use std::process::Command;
use std::time::Instant;

fn sum_of_squares(n: u64) -> u128 {
    (1..=n).map(|i| (i as u128) * (i as u128)).sum()
}

pub fn compare_rust_python_go_performance() -> (f64, f64, f64) {
    // Measure Rust execution time
    let start_time = Instant::now();
    let rust_result = sum_of_squares(10_000_000);
    let rust_duration = start_time.elapsed();
    let rust_elapsed_time_ms = rust_duration.as_secs_f64() * 1000.0;

    // Measure Python execution time
    let start_time = Instant::now();
    let output = Command::new("python")
        .arg("src/sum_squares_perf.py")
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

    // Measure Go execution time
    let start_time = Instant::now();
    let output = Command::new("go")
        .arg("run")
        .arg("src/sum_squares_perf.go")
        .output()
        .expect("Failed to execute Go script");

    let go_duration = start_time.elapsed();
    let go_elapsed_time_ms = go_duration.as_secs_f64() * 1000.0;
    let go_result = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        eprintln!("Go script failed: {:?}", output);
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
    println!("Go Result: {}", go_result.trim());
    println!("Go Time taken: {:.2} ms", go_elapsed_time_ms);
    println!("");

    return (rust_elapsed_time_ms, python_elapsed_time_ms, go_elapsed_time_ms)
}

pub fn display_graph(rust_time: f64, python_time: f64, go_time: f64) {
    println!("Rust (ms)   Python (ms)   Go (ms)");
    println!("---------------------------------");

    println!("{:<10} {:<10} {:<10}", rust_time, python_time, go_time);

    let max_time: f64 = rust_time.max(python_time);
    let rust_bar: usize = (rust_time / max_time * 50.0).round() as usize;
    let python_bar: usize = (python_time / max_time * 50.0).round() as usize;
    let go_bar: usize = (go_time / max_time * 50.0).round() as usize;

    println!("Rust:   [{}]", "*".repeat(rust_bar));
    println!("Python: [{}]", "*".repeat(python_bar));
    println!("Go:     [{}]", "*".repeat(go_bar));
    println!("");
}
