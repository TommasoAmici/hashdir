use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;
use hashdir::calculate_hash;
use ignore::WalkBuilder;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to hash
    dir: PathBuf,
    /// Include hidden files
    #[arg(long)]
    hidden: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut hashes = String::new();
    for result in WalkBuilder::new(args.dir).hidden(args.hidden).build() {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => {
                if entry.path().is_file() {
                    let content = fs::read(entry.path())?;
                    let content_hash = calculate_hash(&content);
                    let file_name = entry
                        .path()
                        .file_name()
                        .expect("Failed to get file name")
                        .to_str()
                        .expect("Failed to convert file name to string");
                    hashes.push_str(format!("{} {}\n", file_name, content_hash).as_str());
                }
            }
            Err(err) => panic!("ERROR: {}", err),
        }
    }

    let mut stdout = io::stdout().lock();
    stdout.write_fmt(format_args!("{}\n", calculate_hash(hashes.as_bytes())))?;

    Ok(())
}
