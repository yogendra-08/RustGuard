use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use crate::scanner::scan_directory;

const BASELINE_FILE: &str = "rustguard_baseline.json";

#[derive(Serialize, Deserialize)]
struct Baseline {
    files: HashMap<String, String>,
}

pub fn create_baseline(dir: &Path) -> io::Result<()> {
    let scanned = scan_directory(dir)?;

    let baseline = Baseline {
        files: scanned.into_iter().collect(),
    };

    let json = serde_json::to_string_pretty(&baseline)
        .map_err(io::Error::other)?;

    fs::write(BASELINE_FILE, json)?;

    println!("Baseline created successfully!");
    println!("Saved to: {}", BASELINE_FILE);
    println!("Files recorded: {}", baseline.files.len());

    Ok(())
}

pub fn verify_directory(dir: &Path) -> io::Result<()> {
    let json = fs::read_to_string(BASELINE_FILE)?;

    let baseline: Baseline =
        serde_json::from_str(&json).map_err(io::Error::other)?;

    let current: HashMap<String, String> =
        scan_directory(dir)?.into_iter().collect();

    let mut modified = 0;
    let mut deleted = 0;
    let mut new_files = 0;

    for (path, old_hash) in &baseline.files {
        match current.get(path) {
            Some(new_hash) if new_hash == old_hash => {
                println!("[OK]      {}", path);
            }
            Some(_) => {
                println!("[CHANGED] {}", path);
                modified += 1;
            }
            None => {
                println!("[DELETED] {}", path);
                deleted += 1;
            }
        }
    }

    for path in current.keys() {
        if !baseline.files.contains_key(path) {
            println!("[NEW]     {}", path);
            new_files += 1;
        }
    }

    println!("\n========== SUMMARY ==========");
    println!("Modified files: {}", modified);
    println!("Deleted files : {}", deleted);
    println!("New files     : {}", new_files);

    if modified == 0 && deleted == 0 && new_files == 0 {
        println!("All files match the baseline.");
    }

    Ok(())
}