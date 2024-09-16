use std::thread::sleep;
use std::time::Duration;
use std::collections::{HashMap, HashSet};
use anyhow::{bail, anyhow};
use anyhow::Context;
use serde_json::Value;
use urlencoding::encode;

#[path ="../users.rs"]
mod users;
use users::USERS;

fn main() {
    let data = std::fs::read_to_string("../Pxls/extras/pixels.log").expect("Failed to read pixels.log");

    let mut board = vec![vec![""; 250]; 250];
    let mut recovered_counts = vec![vec![0; 250]; 250];
    let mut user_counts = HashMap::new();
    for line in data.lines() {
        let mut parts = line.split("\t");
        let Some(time) = parts.next() else {continue};
        let Some(uid) = parts.next() else {continue};
        let Some(username) = parts.next() else {continue};
        let Some(x) = parts.next() else {continue};
        let Some(y) = parts.next() else {continue};
        let Some(color) = parts.next() else {continue};
        let Some(action) = parts.next() else {continue};

        let x = x.parse::<i32>().expect("Failed to parse x");
        let y = y.parse::<i32>().expect("Failed to parse y");

        let Some(faction) = USERS.get(username) else {
            eprintln!("Unknown user: {}", username);
            continue;
        };

        board[x as usize][y as usize] = faction;
        recovered_counts[x as usize][y as usize] += 1;
        *user_counts.entry(username).or_insert(0) += 1;
    }

    let mut stats = HashMap::new();
    for row in board.iter() {
        for faction in row.iter() {
            let count = stats.entry(faction).or_insert(0);
            *count += 1;
        }
    }

    println!("\nPixel counts:");
    for (faction, count) in stats.iter() {
        if faction.is_empty() {
            continue;
        }
        println!("{}: {}", faction, count);
    }

    println!("\nScores:");
    for (faction, count) in stats.iter() {
        match *(*faction) {
            "LH" => println!("{}: {}", faction, (*count as f64 * 1.8).floor()),
            "MRIE" => println!("{}: {}", faction, (*count as f64 * 1.73).floor()),
            "GM" => println!("{}: {}", faction, (*count as f64 * 1.23).floor()),
            "ITI" => println!("{}: {}", faction, (*count as f64 * 1.21).floor()),
            "CFI" => println!("{}: {}", faction, (*count as f64 * 1.21).floor()),
            "MECA" => println!("{}: {}", faction, (*count as f64 * 1.03).floor()),
            "EP" => println!("{}: {}", faction, (*count as f64 * 1.0).floor()),
            _ => continue,
        }
    }

    println!("\nMost recovered pixel:");
    let mut max = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, row) in recovered_counts.iter().enumerate() {
        for (y, count) in row.iter().enumerate() {
            if *count > max {
                max = *count;
                max_x = x;
                max_y = y;
            }
        }
    }
    println!("({}, {}): {}", max_x, max_y, max);

    println!("\nShare of map completed:");
    let mut count = 0;
    for row in board.iter() {
        for faction in row.iter() {
            if !faction.is_empty() {
                count += 1;
            }
        }
    }
    println!("{:.02}%", (count as f64 / 62500.0 * 100.0));

    println!("\nUser count:");
    println!("{:?}", user_counts.len());

    println!("\nBest of each faction:");
    for faction in ["MECA", "MRIE", "CFI", "EP", "GM", "ITI", "LH"] {
        let mut best = 0;
        let mut best_user = "";
        for (user, user_count) in user_counts.iter() {
            if *USERS.get(user).unwrap() == faction && *user_count > best {
                best = *user_count;
                best_user = user;
            }
        }
        println!("{}: {} ({})", faction, best_user, best);
    }
}
