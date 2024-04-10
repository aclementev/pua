use clap::Parser;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// A CLI for interactive structured log exploration
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the log file
    path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.path);
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines().take(10).flatten() {
        let m: HashMap<String, serde_json::value::Value> = serde_json::from_str(&line)?;
        data.push(m);
    }

    println!("Read {} lines", data.len());

    Ok(())
}

/// Handles log exploration
struct Pua {}
