use std::process::Command;
use std::time::Instant;

fn sum_of_squares(n: u64) -> u128 {
    (1..=n).map(|i| (i as u128) * (i as u128)).sum()
}

pub fn compare_performance() {
    // Measure Rust execution time
    let start_time = Instant::now();
    let result = sum_of_squares(10_000_000);
    let rust_duration = start_time.elapsed();
    let rust_elapsed_time_ms = rust_duration.as_secs_f64() * 1000.0;
    
    // Print Rust output
    println!("Rust Result: {}", result);
    println!("Rust Time taken: {:.2} ms", rust_elapsed_time_ms);

    // Measure Python execution time
    let start_time = Instant::now();
    let output = Command::new("python")
        .arg("sum_of_squares_perf_test.py")
        .output()
        .expect("Failed to execute Python script");

    let python_duration = start_time.elapsed();
    let python_elapsed_time_ms = python_duration.as_secs_f64() * 1000.0;

    // Print Python script output
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Python Output:\n{}", output_str);
    } else {
        eprintln!("Python script failed: {:?}", output);
    }

    println!("Python Time taken: {:.2} ms", python_elapsed_time_ms);
}
