use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Opt {
    #[arg(index = 1)]
    /// Directory to search for stale directories
    pub directory: String,

    #[arg(short, long, default_value = "14")]
    /// Number of days after which the directory is considered stale
    pub days: u64,
}

pub fn get_args() -> Opt {
    Opt::parse()
}