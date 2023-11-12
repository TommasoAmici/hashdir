use ignore::WalkBuilder;
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use xxhash_rust::xxh3::xxh3_64;

fn calculate_hash(input: &[u8]) -> u64 {
    xxh3_64(input)
}

pub fn hashdir(directory: PathBuf, hidden_files: bool) -> Result<u64, Error> {
    let mut hashes = String::new();
    for result in WalkBuilder::new(directory).hidden(hidden_files).build() {
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
    Ok(calculate_hash(hashes.as_bytes()))
}
