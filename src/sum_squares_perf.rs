use std::process::Command;
use std::time::{Duration, Instant};
use serde::Deserialize;

fn sum_of_squares(n: u64) -> u128 {
    (1..=n).map(|i: u64| (i as u128) * (i as u128)).sum()
}

#[derive(Deserialize)]
struct LanguageOutput {
    result: u128,
    elapsed_time_ms: f64,
}

fn get_rust_results() -> LanguageOutput {
    let start_time: Instant = Instant::now();
    let rust_result: u128 = sum_of_squares(10_000_000);
    let rust_duration: Duration = start_time.elapsed();
    let rust_elapsed_time_ms: f64 = rust_duration.as_secs_f64() * 1000.0;
    
    return LanguageOutput {
        result: rust_result,
        elapsed_time_ms: rust_elapsed_time_ms,
    }
}

fn get_python_results() -> LanguageOutput {
    let python_output = Command::new("python")
        .arg("src/sum_squares_perf.py")
        .output()
        .expect("Failed to execute Python script");
        
    if python_output.status.success() {
        let output_str = String::from_utf8_lossy(&python_output.stdout).to_string();
        let python_output: LanguageOutput = serde_json::from_str(&output_str)
            .expect("Failed to parse JSON from Python script");

        return python_output;
    } else {
        eprintln!("Python script failed: {:?}", python_output);
        return LanguageOutput {
            result: 0,
            elapsed_time_ms: 0.0,
        }
    }
}


fn get_go_results() -> LanguageOutput {
    let go_output = Command::new("go")
        .arg("run")
        .arg("src/sum_squares_perf.go")
        .output()
        .expect("Failed to execute Go script");
        
    if go_output.status.success() {
        let output_str = String::from_utf8_lossy(&go_output.stdout).to_string();
        let go_output: LanguageOutput = serde_json::from_str(&output_str)
            .expect("Failed to parse JSON from Go script");

        return go_output;
    } else {
        eprintln!("Go script failed: {:?}", go_output);
        return LanguageOutput {
            result: 0,
            elapsed_time_ms: 0.0,
        }
    }
}


pub fn compare_rust_python_go_performance() -> (f64, f64, f64) {
    // Get Results
    let rust_output = get_rust_results();
    let python_output = get_python_results();
    let go_output = get_go_results();

    // Print output
    println!("");
    println!("Rust Result: {}", rust_output.result);
    println!("Rust Time taken: {:.2} ms", rust_output.elapsed_time_ms);
    println!("");
    println!("Python Result: {}", python_output.result);
    println!("Python Time taken: {:.2} ms", python_output.elapsed_time_ms);
    println!("");
    println!("Go Result: {}", go_output.result);
    println!("Go Time taken: {:.2} ms", go_output.elapsed_time_ms);
    println!("");

    return (rust_output.elapsed_time_ms, python_output.elapsed_time_ms, go_output.elapsed_time_ms)
}

pub fn display_graph(rust_time: f64, python_time: f64, go_time: f64) {
    println!("Rust (ms)   Python (ms)   Go (ms)");
    println!("---------------------------------");

    println!("{:<10.2} {:<10.2} {:<10.2}", rust_time, python_time, go_time);

    let max_time: f64 = rust_time.max(python_time);
    let rust_bar: usize = (rust_time / max_time * 50.0).round() as usize;
    let python_bar: usize = (python_time / max_time * 50.0).round() as usize;
    let go_bar: usize = (go_time / max_time * 50.0).round() as usize;

    println!("Rust:   [{}]", "*".repeat(rust_bar));
    println!("Python: [{}]", "*".repeat(python_bar));
    println!("Go:     [{}]", "*".repeat(go_bar));
    println!("");
}
