use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Write};

mod constants;
mod sum_squares_perf_test;
mod introductions;

fn write_to_file(content: &str) {
    // Get the data directory from constants
    let dir_path = constants::data_dir();
    println!("Data directory: {:?}", dir_path);

    // Construct the file path
    let file_path = dir_path.join("results.txt");
    println!("File path: {:?}", file_path);

    // Ensure the directory exists
    create_dir_all(&dir_path).expect("Failed to create data directory");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open results file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to results file");
}

fn main() {
    // Introductions
    let do_introductions = false;
    if do_introductions {
        introductions::do_introductions()
    }

    // Performance Tests
    let (rust_time, python_time) = sum_squares_perf_test::compare_rust_python_performance();
    println!("Rust execution time: {} ms", rust_time);
    println!("Python execution time: {} ms", python_time);

    // Write results to file with a timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let result_entry = format!(
        "{}, Rust: {:.2} ms, Python: {:.2} ms\n",
        timestamp, rust_time, python_time
    );
    write_to_file(&result_entry);

    // Graph results
    sum_squares_perf_test::display_graph(rust_time, python_time);
}
