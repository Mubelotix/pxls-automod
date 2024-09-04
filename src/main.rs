use std::thread::sleep;
use std::time::Duration;

use anyhow::bail;
use anyhow::Context;
use serde_json::Value;

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

fn run() -> anyhow::Result<()> {
    let users = list_users()?;
    println!("Users: {:?}", users);

    Ok(())
}

fn main() {
    let token = std::env::var("PXLS_TOKEN").expect("missing PXLS_TOKEN variable");

    println!("Running");

    loop {
        if let Err(e) = run() {
            eprintln!("Error: {}", e)
        }

        sleep(Duration::from_secs(60));
    }
}
