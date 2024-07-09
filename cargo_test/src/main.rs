
fn main() {
    let print_string: &str = "Hello, world!";
    println!("{}", print_string);
    
    let mut mutable_print_string = "Hello, world Mutable";
    println!("Mutable print string: {}", mutable_print_string);
    
    mutable_print_string = "Hello, world Mutable Changed";
    println!("Mutable print string: {}", mutable_print_string);
}
