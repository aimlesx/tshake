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

fn discover_projects(path: &String, cfg: &config::Config) -> impl Iterator<Item = DirEntry> {
    let detect: HashSet<String> = cfg
        .projects
        .values()
        .filter_map(|project| project.detect.clone())
        .flatten()
        .collect();

    let dirs = WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir() && !should_skip(e));

    let is_project_root = move |entry: &DirEntry| {
        let dir = entry.path();
        let items = dir.read_dir().unwrap();
        let items: Vec<String> = items
            .filter_map(|item| item.ok())
            .filter_map(|item| item.file_name().into_string().ok())
            .collect();
        items.iter().any(|item| detect.contains(item))
    };

    dirs.map_while(|entry| entry.ok())
        .filter(is_project_root)
}

fn main() {
    let args = get_args();
    let cfg = get_config();

    let mut count: u64 = 0;

    let project_roots = discover_projects(&args.directory, &cfg);

    for entry in project_roots {
        let path = entry.path().to_string_lossy().to_string();
        println!("{}", path);
        count += 1;
    }

    println!("Number of projects discovered in the path: {}", count);
}
