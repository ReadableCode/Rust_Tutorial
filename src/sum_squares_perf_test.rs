use std::process::Command;
use std::time::Instant;

fn sum_of_squares(n: u64) -> u128 {
    (1..=n).map(|i| (i as u128) * (i as u128)).sum()
}

pub fn compare_rust_python_performance() -> (f64, f64) {
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

    return (rust_elapsed_time_ms, python_elapsed_time_ms);
}

pub fn display_graph(rust_time: f64, python_time: f64) {
    println!("Rust (ms)   Python (ms)");
    println!("-----------------------");

    println!("{:<10} {:<10}", rust_time, python_time);

    let max_time = rust_time.max(python_time);
    let rust_bar = (rust_time / max_time * 50.0).round() as usize;
    let python_bar = (python_time / max_time * 50.0).round() as usize;

    println!("Rust:   [{}]", "*".repeat(rust_bar));
    println!("Python: [{}]", "*".repeat(python_bar));
    println!("");
}
