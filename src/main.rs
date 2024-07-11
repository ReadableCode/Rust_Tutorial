use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use csv::Writer;
use hostname::get;
use std::thread;

mod constants;
mod sum_squares_perf;
mod introductions;

fn thread_demo() {
    // Number of threads to create
    let num_threads = 5;

    // Vector to hold the thread handles
    let mut handles = vec![];

    for i in 0..num_threads {
        // Spawn a new thread
        let handle = thread::spawn(move || {
            println!("Thread {} is starting.", i);
            thread::sleep(Duration::from_secs(1));
            println!("Thread {} is done.", i);
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads are done.");
}

fn write_to_log_file(content: &str) {
    // Get the data directory from constants
    let dir_path = constants::data_dir();

    // Construct the file path
    let file_path = dir_path.join("results.txt");

    // Ensure the directory exists
    create_dir_all(&dir_path).expect("Failed to create data directory");

    // Write to file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open results file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to results file");
}

fn write_to_csv_file(timestamp: u64, hostname: String, rust_time: f64, python_time: f64, go_time: f64) {
    // Get the data directory from constants
    let dir_path = constants::data_dir();

    // Construct the file path
    let file_path = dir_path.join("results.csv");

    // Ensure the directory exists
    create_dir_all(&dir_path).expect("Failed to create data directory");

    // Write to file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open results file");

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&[
        timestamp.to_string(),
        hostname.clone(),
        format!("{:.2}", rust_time),
        format!("{:.2}", python_time),
        format!("{:.2}", go_time),
    ])
    .expect("Failed to write to CSV file");
    
    wtr.flush().expect("Failed to flush CSV writer");
}

fn main() {
    // Hostname
    let hostname: String = get().unwrap().into_string().unwrap();
    println!("Hostname: {}", hostname);
    
    // Do Thread Demo
    let do_thread_demo = true;
    if do_thread_demo {
        thread_demo();
    }

    // Introductions
    let do_introductions = false;
    if do_introductions {
        introductions::do_introductions()
    }

    // Performance Tests
    let do_performance_tests = false;
    if do_performance_tests {
        let (rust_time, python_time, go_time) = sum_squares_perf::compare_rust_python_go_performance();

        // Write results to log file with a timestamp
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Write to log file
        let result_log_entry = format!(
            "{}, {}, Rust: {:.2} ms, Python: {:.2} ms, Go: {:.2} ms\n",
            timestamp, hostname, rust_time, python_time, go_time
        );
        write_to_log_file(&result_log_entry);
        
        // Write to csv file
        write_to_csv_file(timestamp, hostname, rust_time, python_time, go_time);

        // Graph results
        sum_squares_perf::display_graph(rust_time, python_time, go_time);
    }
}
