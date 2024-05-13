mod config;
mod args;

use std::collections::HashSet;
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

    let detect: HashSet<String> = cfg
        .projects
        .values()
        .filter_map(|project| project.detect.clone())
        .flatten()
        .collect();

    let mut count: u64 = 0;

    let dirs = WalkDir::new(&args.directory)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir() && !should_skip(e));

    let is_project_root = |entry: &DirEntry| {
        let dir = entry.path();
        let items = dir.read_dir().unwrap();
        let items: Vec<String> = items
            .filter_map(|item| item.ok())
            .filter_map(|item| item.file_name().into_string().ok())
            .collect();
        items.iter().any(|item| detect.contains(item))
    };

    let project_roots = dirs
        .map_while(|entry| entry.ok())
        .filter(is_project_root);

    for entry in project_roots {
        let path = entry.path().to_string_lossy().to_string();
        println!("{}", path);
        count += 1;
    }

    println!("Number of projects discovered in the path: {}", count);
}
