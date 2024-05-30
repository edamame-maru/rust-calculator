use std::io;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    print!("Enter number of operands: ");
    io::stdout().flush().unwrap();

    let mut operands = String::new();

    io::stdin()
        .read_line(&mut operands)
        .expect("Failed to read line");

    let mut operands: i32 = operands.trim().parse().expect("Failed to parse to integer.");
    
    let operators: i32 = operands - 2; 

    println!("{} operands, therefore {} operators.", &mut operands, operators);
}
