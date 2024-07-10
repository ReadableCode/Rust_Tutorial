use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    current_dir.join("data")
}