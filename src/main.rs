use clap::Parser;
use walkdir::WalkDir;

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

fn is_project_root(entry: &walkdir::DirEntry) -> bool {
    if !entry.file_type().is_dir() { return false; }

    false
}

fn main() {
    let opt = Opt::parse();
    let mut count: u64 = 0;

    let known_folders: Vec<&str> = include_str!("../known_folders.txt").lines().collect();

    let mut iter = WalkDir::new(&opt.directory).into_iter();

    while let Some(entry) = iter.next() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            },
        };

        if !entry.file_type().is_dir() { continue; }

        let filename = &entry.file_name().to_string_lossy().to_string();

        if known_folders.contains(&filename.as_str()) {
            iter.skip_current_dir();
        } else {
            count += 1;
        }
    }

    println!("Number of directories discovered in the path: {count}");
}
