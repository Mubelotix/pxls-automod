use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
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
    }

    let mut stats = HashMap::new();
    for row in board.iter() {
        for faction in row.iter() {
            let count = stats.entry(faction).or_insert(0);
            *count += 1;
        }
    }
    for (faction, count) in stats.iter() {
        if faction.is_empty() {
            continue;
        }
        println!("{}: {}", faction, count);
    }
}
