use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use anyhow::bail;
use anyhow::Context;
use serde_json::Value;
use urlencoding::encode;

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
    None,
    Ban { reason: String },
    Rename { new_name: String },
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
        return Ok(ActionRequired::None);
    }

    // Skip if user is banned
    let banned = res.get("banned").context("Failed to find banned")?;
    let banned = banned.as_bool().context("Banned is not a boolean")?;
    if banned {
        return Ok(ActionRequired::None);
    }

    // Get logins
    let logins = res.get("logins").context("Failed to find logins")?;
    let logins = logins.as_array().context("Logins is not an array")?;
    if logins.len() != 1 {
        return Ok(ActionRequired::Ban {
            reason: String::from("Vous ne devez avoir qu'exactement un seul login. Contactez un admin."),
        });
    }
    let login = logins[0].as_object().context("Login is not an object")?;
    let service_id = login.get("serviceID").context("Failed to find serviceID")?;
    let service_id = service_id.as_str().context("ServiceID is not a string")?;
    if service_id != "cas" {
        return Ok(ActionRequired::Ban {
            reason: String::from("Vous ne devez vous connecter qu'avec le CAS. Contactez un admin."),
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

fn permaban(token: &'static str, username: &str, reason: &str) -> anyhow::Result<()> {
    // Send request
    let rep = minreq::post("https://pixelwar.insa.lol/admin/permaban")
        .with_header("Cookie", format!("pxls-token={token}"))
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(format!("username={}&reason={}", encode(username), encode(reason)))
        .send()
        .context("Failed to permaban user")?;
    if rep.status_code != 200 {
        bail!("Failed to permaban user: {}", rep.status_code);
    }

    Ok(())
}

fn get_factions(token: &'static str) -> anyhow::Result<HashMap<String, usize>> {
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
    let factions: HashMap<String, usize> = factions
        .iter()
        .map(|faction| {
            let name = faction.get("name").and_then(|n| n.as_str()).context("Failed to find faction name")?;
            let id = faction.get("color").and_then(|c| c.as_u64()).context("Failed to find faction color")?;
            Ok((name.to_string(), id as usize))
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

fn prepare(token: &'static str) -> anyhow::Result<HashMap<String, usize>> {
    const EXPECTED_FACTIONS: &[(&str, usize)] = &[
        ("MECA", 0xffea00),
        ("MRIE", 0x59ff21),
        ("CFI", 0xff2200),
        ("EP", 0xff8ac0),
        ("GM", 0x0600bd),
        ("ITI", 0x00fffb),
        ("GC", 0xff7700)
    ];

    let mut existing_factions = get_factions(token)?;
    println!("Factions: {existing_factions:?}");

    let mut changed = false;
    for (faction, color) in EXPECTED_FACTIONS {
        if !existing_factions.contains_key(*faction) {
            println!("Creating faction {faction}");
            create_faction(token, faction, *color)?;
            changed = true;
        }
    }

    if changed {
        existing_factions = get_factions(token)?;
        println!("Factions: {existing_factions:?}");
    }

    Ok(existing_factions)
}

fn run(token: &'static str) -> anyhow::Result<()> {
    let users = list_users()?;
    println!("Users: {:?}", users);

    for user in users {
        match check_user_login(token, &user) {
            Ok(ActionRequired::Rename { new_name }) => match change_name(token, &user, &new_name) {
                Ok(_) => println!("User {user} renamed to {new_name}"),
                Err(e) => eprintln!("Error renaming user {user}: {}", e),
            },
            Ok(ActionRequired::Ban { reason }) => match permaban(token, &user, &reason) {
                Ok(_) => println!("User {user} banned: {reason}"),
                Err(e) => eprintln!("Error banning user {user}: {}", e),
            },
            Ok(ActionRequired::None) => {},
            Err(e) => eprintln!("Error checking user {}: {}", user, e),
        }
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
