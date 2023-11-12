use xxhash_rust::xxh3::xxh3_64;

pub fn calculate_hash(input: &[u8]) -> u64 {
    xxh3_64(input)
}
