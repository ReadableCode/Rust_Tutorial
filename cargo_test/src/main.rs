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
    println!("How old are you?");
    
    let mut your_age_string: String = String::new();
    io::stdin().read_line(&mut your_age_string).expect("failed to read age");
    
    let your_age_int: i8 = your_age_string.trim().parse().unwrap();
    println!("You are going to be {} years old next year", your_age_int + 1);
    
    // find out if the age is even or not next year
    let is_even: bool = (your_age_int % 2) == 0;
    println!("Your age: {}, is an even number = {}", your_age_int, is_even);
    
    if is_even {
        println!("That is even")
    }
    else {
        println!("That is odd")
    }
    
}
