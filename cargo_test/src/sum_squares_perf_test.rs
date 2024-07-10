use std::fs::{OpenOptions, create_dir_all};
use std::io::{BufRead, Write};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};


fn sum_of_squares(n: u64) -> u128 {
    (1..=n).map(|i| (i as u128) * (i as u128)).sum()
}

fn write_to_file(content: &str) {
    let dir_path = "../data";
    let file_path = format!("{}/results.txt", dir_path);

    // Ensure the directory exists
    create_dir_all(dir_path).expect("Failed to create data directory");


    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open results file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to results file");
}

pub fn compare_performance() {
    // Measure Rust execution time
    let start_time = Instant::now();
    let rust_result = sum_of_squares(10_000_000);
    let rust_duration = start_time.elapsed();
    let rust_elapsed_time_ms = rust_duration.as_secs_f64() * 1000.0;

    // Measure Python execution time
    let start_time = Instant::now();
    let output = Command::new("python")
        .arg("src/sum_squares_perf_test.py")
        .output()
        .expect("Failed to execute Python script");

    let python_duration = start_time.elapsed();
    let python_elapsed_time_ms = python_duration.as_secs_f64() * 1000.0;
    let python_result = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        eprintln!("Python script failed: {:?}", output);
        String::new()
    };

    // Print output
    println!("");
    println!("Rust Result: {}", rust_result);
    println!("Rust Time taken: {:.2} ms", rust_elapsed_time_ms);
    println!("");
    println!("Python Result: {}", python_result.trim());
    println!("Python Time taken: {:.2} ms", python_elapsed_time_ms);
    println!("");

    // Write results to file with a timestamp
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let result_entry = format!(
        "{}, Rust: {:.2} ms, Python: {:.2} ms\n",
        timestamp, rust_elapsed_time_ms, python_elapsed_time_ms
    );
    write_to_file(&result_entry);
}


pub fn display_graph() {

    let dir_path = "../data";
    let file_path = format!("{}/results.txt", dir_path);

    let file = std::fs::File::open(file_path).expect("Failed to open results file");
    let reader = std::io::BufReader::new(file);

    let mut results = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(", ").collect();
            if parts.len() == 3 {
                let timestamp: u64 = parts[0].parse().unwrap();
                let rust_time: f64 = parts[1].split(" ").nth(1).unwrap().parse().unwrap();
                let python_time: f64 = parts[2].split(" ").nth(1).unwrap().parse().unwrap();
                results.push((timestamp, rust_time, python_time));
            }
        }
    }
    println!("Timestamp          Rust (ms)   Python (ms)");
    println!("-----------------------------------------");

    for (timestamp, rust_time, python_time) in results {
        println!("{:<18} {:<10} {:<10}", timestamp, rust_time, python_time);

        let max_time = rust_time.max(python_time);
        let rust_bar = (rust_time / max_time * 50.0).round() as usize;
        let python_bar = (python_time / max_time * 50.0).round() as usize;

        println!("Rust:   [{}]", "*".repeat(rust_bar));
        println!("Python: [{}]", "*".repeat(python_bar));
        println!("");
    }
}
