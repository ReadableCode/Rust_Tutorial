use std::io;

fn main() {
    let print_string: &str = "Hello, world!";
    println!("{}", print_string);
    
    let mut mutable_print_string = "Hello, world Mutable";
    println!("Mutable print string: {}", mutable_print_string);
    
    mutable_print_string = "Hello, world Mutable Changed";
    println!("Mutable print string: {}", mutable_print_string);
    
    println!("What is your name?");
    let mut your_name = String::new();
    io::stdin().read_line(&mut your_name).expect("failed to read line");
    
    println!("Nice to meet you, {}", your_name);
}
