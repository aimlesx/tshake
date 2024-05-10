use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(version, about)]
struct Opt {
    #[arg(index = 1)]
    /// Directory to search for stale directories
    directory: String,

    #[arg(short, long, default_value = "14")]
    /// Number of days after which the directory is considered stale
    days: u64,
}

fn should_skip(entry: &DirEntry) -> bool {
    let known_folders: Vec<&str> = include_str!("../known_folders.txt").lines().collect();

    entry
        .file_name()
        .to_str()
        .map_or(false, |name| known_folders.contains(&name))
}

fn main() {
    let opt = Opt::parse();
    let mut count: u64 = 0;

    let iter = WalkDir::new(&opt.directory)
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
