use clap::Parser;
use std::{fs, process};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the new directory
    paths: Vec<String>,

    /// Creates parent directories if needed
    #[arg(short, long)]
    parents: bool,

    /// Prints a message for each created directory
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if args.parents {
        // Creates parent directories
        for path in args.paths {
            if let Err(e) = fs::create_dir_all(&path) {
                eprintln!("mkdir: Cannot create directory \"{}\": {}", path, e);
                process::exit(1);
            }

            if args.verbose {
                println!("mkdir: Created directory \"{}\"", path);
            }
        }
    } else {
        for path in args.paths {
            if let Err(e) = fs::create_dir(&path) {
                eprintln!("mkdir: Cannot create directory \"{}\": {}", path, e);
                process::exit(1);
            }

            if args.verbose {
                println!("mkdir: Created directory \"{}\"", path);
            }
        }
    }
}
