use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;
use hashdir::hashdir;

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
    let mut stdout = io::stdout().lock();
    stdout.write_fmt(format_args!("{}\n", hashdir(args.dir, args.hidden)?))?;
    Ok(())
}
