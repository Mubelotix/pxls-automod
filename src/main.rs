use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use anyhow::{bail, anyhow};
use anyhow::Context;
use serde_json::Value;
use urlencoding::encode;

mod users;
use users::{USERS, FACTIONS};

fn list_users() -> anyhow::Result<Vec<String>> {
    // Send request
    let rep = minreq::get("https://pixelwar.insa.lol/stats/stats.json")
        .send()
        .context("Failed to load stats")?;
    if rep.status_code != 200 {
        bail!("Failed to load stats: {}", rep.status_code);
    }

    // Parse response
    let stats: Value = rep.json().context("Failed to parse stats")?;

    // Get user list
    let canvas = stats
        .get("toplist")
        .and_then(|t| t.get("canvas"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find canvas toplist.canvas"))?;

    // For each item get the username
    let users = canvas
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Canvas toplist is not an array"))?
        .iter()
        .map(|item| {
            item.get("username")
                .and_then(|u| u.as_str())
                .ok_or_else(|| anyhow::anyhow!("Failed to find username"))
                .map(|u| u.to_string())
        })
        .collect::<anyhow::Result<Vec<String>>>()?;

    Ok(users)
}

#[derive(Debug)]
enum ActionRequired {
    Skip,
    None,
    Ban { reason: String },
    Rename { new_name: String },
    JoinFaction { fid: usize },
}

impl ActionRequired {
    fn exec(self, token: &'static str, username: &str) -> String {
        match self {
            ActionRequired::Skip => {},
            ActionRequired::None => {},
            ActionRequired::Ban { reason } => match permaban(token, username, &reason) {
                Ok(_) => println!("User {username} banned: {reason}"),
                Err(e) => eprintln!("Error banning user {username}: {}", e),
            },
            ActionRequired::Rename { new_name } => match change_name(token, username, &new_name) {
                Ok(_) => {
                    println!("User {username} renamed to {new_name}");
                    return new_name.to_owned();
                },
                Err(e) => eprintln!("Error renaming user {username}: {}", e),
            },
            ActionRequired::JoinFaction { fid } => match change_faction(token, username, fid) {
                Ok(_) => println!("User {username} joined faction {fid}"),
                Err(e) => eprintln!("Error joining faction {fid}: {}", e),
            },
        }
        username.to_owned()
    }
}

fn check_user_login(token: &'static str, username: &str) -> anyhow::Result<ActionRequired> {
    // Send request
    let rep = minreq::post("https://pixelwar.insa.lol/admin/check")
        .with_header("Cookie", format!("pxls-token={token}"))
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(format!("username={}", encode(username)))
        .send()
        .context("Failed to check user")?;
    if rep.status_code != 200 {
        bail!("Failed to check user: {}", rep.status_code);
    }

    // Parse response
    let res: Value = rep.json().context("Failed to parse check response")?;

    // Skip if user is admin
    let roles = res.get("roles").context("Failed to find roles")?;
    let roles = roles.as_array().context("Roles is not an array")?;
    let roles = roles
        .iter()
        .map(|role| {
            role.get("id")
                .and_then(|id| id.as_str())
                .ok_or_else(|| anyhow::anyhow!("Failed to find role id"))
                .map(|id| id.to_string())
        })
        .collect::<anyhow::Result<Vec<String>>>()?;
    let is_more_than_user = roles.iter().any(|role| role != "guest" && role != "user");
    if is_more_than_user {
        return Ok(ActionRequired::Skip);
    }

    // Skip if user is banned
    let banned = res.get("banned").context("Failed to find banned")?;
    let banned = banned.as_bool().context("Banned is not a boolean")?;
    if banned {
        return Ok(ActionRequired::Skip);
    }

    // Get logins
    let logins = res.get("logins").context("Failed to find logins")?;
    let logins = logins.as_array().context("Logins is not an array")?;
    if logins.len() != 1 {
        return Ok(ActionRequired::Ban {
            reason: String::from("Vous ne devez avoir qu'exactement un seul login. Contactez moi (cf les infos)."),
        });
    }
    let login = logins[0].as_object().context("Login is not an object")?;
    let service_id = login.get("serviceID").context("Failed to find serviceID")?;
    let service_id = service_id.as_str().context("ServiceID is not a string")?;
    if service_id != "cas" {
        return Ok(ActionRequired::Ban {
            reason: String::from("Vous ne devez vous connecter qu'avec le CAS. Contactez moi (cf les infos)."),
        });
    }

    // Get expected username
    let service_user_id = login.get("serviceUserID").context("Failed to find serviceUserID")?;
    let service_user_id = service_user_id.as_str().context("ServiceUserID is not a string")?;
    if service_user_id != username {
        return Ok(ActionRequired::Rename {
            new_name: service_user_id.to_string(),
        });
    }

    Ok(ActionRequired::None)
}

fn check_user_factions(old_username: &String, username: &str, factions: &HashMap<String, (usize, Vec<String>)>) -> anyhow::Result<ActionRequired> {
    // Get user factions
    let user_factions = factions.values().filter(|(_,members)| members.contains(old_username)).map(|(fid, _)| *fid).collect::<Vec<_>>();
    if user_factions.len() > 1 {
        return Ok(ActionRequired::Ban {
            reason: String::from("Vous ne devez avoir qu'une seule faction. Contactez moi (cf les infos)."),
        });
    }
    let user_fid = user_factions.first().copied();

    // Get expected faction
    let expected_faction = match USERS.get(username).copied() {
        Some(faction) => faction,
        None => return Ok(ActionRequired::Ban {
            reason: String::from("Vous n'etes pas inscrits en 3eme année. Si c'est une erreur et que vous etes dans une situation particuliere, contactez moi (cf les infos)."),
        }),
    };
    let expected_fid = factions.get(expected_faction).map(|(fid,_)| *fid).ok_or_else(|| anyhow!("Faction inconnue: {expected_faction}"))?;

    // When user is in the right faction
    if user_fid == Some(expected_fid) {
        return Ok(ActionRequired::None);
    }

    // When user is not in any faction
    if user_fid.is_none() {
        return Ok(ActionRequired::JoinFaction {
            fid: expected_fid,
        });
    }

    // When user is in the wrong faction
    Ok(ActionRequired::Ban {
        reason: String::from("Vous n'etes pas dans la bonne faction. Contactez moi (cf les infos)."),
    })
}

fn change_name(token: &'static str, username: &str, new_name: &str) -> anyhow::Result<()> {
    // Send request
    let rep = minreq::post("https://pixelwar.insa.lol/admin/forceNameChange")
        .with_header("Cookie", format!("pxls-token={token}"))
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(format!("user={}&newName={}", encode(username), encode(new_name)))
        .send()
        .context("Failed to rename user")?;
    if rep.status_code != 200 {
        bail!("Failed to rename user: {}", rep.status_code);
    }

    Ok(())
}

fn change_faction(token: &'static str, username: &str, fid: usize) -> anyhow::Result<()> {
    // Send request
    let rep = minreq::post("https://pixelwar.insa.lol/admin/faction/join")
        .with_header("Cookie", format!("pxls-token={token}"))
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(format!("user={}&fid={}", encode(username), fid))
        .send()
        .context("Failed to change faction")?;
    if rep.status_code != 200 {
        bail!("Failed to change faction: {}", rep.status_code);
    }

    Ok(())
}

fn permaban(token: &'static str, username: &str, reason: &str) -> anyhow::Result<()> {
    // Send request
    let rep = minreq::post("https://pixelwar.insa.lol/admin/permaban")
        .with_header("Cookie", format!("pxls-token={token}"))
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(format!("username={}&reason={}&rollback_time=1000000", encode(username), encode(reason)))
        .send()
        .context("Failed to permaban user")?;
    if rep.status_code != 200 {
        bail!("Failed to permaban user: {}", rep.status_code);
    }

    Ok(())
}

fn get_factions(token: &'static str) -> anyhow::Result<HashMap<String, (usize, Vec<String>)>> {
    // Send request
    let rep = minreq::get("https://pixelwar.insa.lol/api/v1/profile?username=automod")
        .with_header("Cookie", format!("pxls-token={token}"))
        .send()
        .context("Failed to load factions")?;
    if rep.status_code != 200 {
        bail!("Failed to load factions: {}", rep.status_code);
    }

    // Get factions
    let data: Value = rep.json().context("Failed to parse factions")?;
    let user = data.get("user").context("Failed to find user")?;
    let factions = user.get("factions").context("Failed to find factions")?;
    let factions = factions.as_array().context("Factions is not an array")?;
    let factions: HashMap<String, (usize, Vec<String>)> = factions
        .iter()
        .map(|faction| {
            let name = faction.get("name").and_then(|n| n.as_str()).context("Failed to find faction name")?;
            let id = faction.get("id").and_then(|c| c.as_u64()).context("Failed to find faction id")?;
            let members = faction.get("members").context("Failed to find members")?;
            let members = members.as_array().context("Members is not an array")?;
            let members: Vec<String> = members
                .iter()
                .map(|member| {
                    member.get("name")
                        .and_then(|u| u.as_str())
                        .ok_or_else(|| anyhow::anyhow!("Failed to find name"))
                        .map(|u| u.to_string())
                })
                .collect::<anyhow::Result<_>>()?;
            Ok((name.to_string(), (id as usize, members)))
        })
        .collect::<anyhow::Result<_>>()?;

    Ok(factions)
}

fn create_faction(token: &'static str, faction: &str, color: usize) -> anyhow::Result<()> {
    // Send request
    let rep = minreq::post("https://pixelwar.insa.lol/factions")
        .with_header("Cookie", format!("pxls-token={token}"))
        .with_header("Content-Type", "application/json")
        .with_body(format!("{{\"name\":{faction:?},\"tag\":{faction:?},\"color\":{color}}}"))
        .send()
        .context("Failed to create faction")?;
    if rep.status_code != 200 {
        bail!("Failed to create faction: {} {}", rep.status_code, rep.as_str().unwrap_or_default());
    }

    Ok(())
}

fn prepare(token: &'static str) -> anyhow::Result<()> {
    let existing_factions = get_factions(token)?;
    println!("Factions: {existing_factions:?}");

    for (faction, color) in FACTIONS {
        if !existing_factions.contains_key(*faction) {
            println!("Creating faction {faction}");
            create_faction(token, faction, *color)?;
        }
    }

    Ok(())
}

fn run(token: &'static str) -> anyhow::Result<()> {
    let users = list_users()?;
    println!("Users: {:?}", users);

    let factions = get_factions(token)?;
    println!("Factions: {:?}", factions);

    for username in users {
        // Check user logins
        let action_required = match check_user_login(token, &username) {
            Ok(ActionRequired::Skip) => continue,
            Ok(action) => action,
            Err(e) => {
                eprintln!("Error checking user {}: {}", username, e);
                continue;
            },
        };
        let new_username = action_required.exec(token, &username);

        // Check user factions
        let action_required = match check_user_factions(&username, &new_username, &factions) {
            Ok(action) => action,
            Err(e) => {
                eprintln!("Error checking user factions {}: {}", username, e);
                continue;
            },
        };
        action_required.exec(token, &new_username);
    }

    Ok(())
}

fn main() {
    let token = std::env::var("PXLS_TOKEN").expect("missing PXLS_TOKEN variable");
    let token = token.leak();

    println!("Preparing");
    prepare(token).expect("Failed to prepare");

    println!("Running");
    loop {
        if let Err(e) = run(token) {
            eprintln!("Error: {}", e)
        }

        sleep(Duration::from_secs(60));
    }
}
