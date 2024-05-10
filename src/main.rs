mod config;
mod args;

use config::get_config;
use args::get_args;
use walkdir::{DirEntry, WalkDir};

fn should_skip(entry: &DirEntry) -> bool {
    let known_folders: Vec<&str> = include_str!("../known_folders.txt").lines().collect();

    entry
        .file_name()
        .to_str()
        .map_or(false, |name| known_folders.contains(&name))
}

fn main() {
    let args = get_args();
    let cfg = get_config();

    let mut count: u64 = 0;

    let iter = WalkDir::new(&args.directory)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir() && !should_skip(e));

    for entry in iter {
        if let Ok(entry) = entry {
            let path = entry.path().to_string_lossy().to_string();
            println!("{}", path);
            count += 1;
        }
    }

    println!("Number of directories discovered in the path: {}", count);
}
