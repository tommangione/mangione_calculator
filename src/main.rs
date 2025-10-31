//this calculator will do two things to begin with: calculate nth fibonacci numbers
//  and calculate F to C and C to F

use::std::io;

fn main() {
    println!("Select the function you want.\nEnter 1 for Fibonacci number calculator\nEnter 2 for Celsius to Farenheit\nEnter 3 for Farenheit to Celsius");
    let mut selection = String::new();
    
    io::stdin().read_line(&mut selection).expect("Failed to read line.");
    
    let selection: u8 = selection.trim().parse().expect("");

    if selection == 1 {
        fib();
    } else if selection == 2 || selection == 3 {
        println!("Not yet implemented.");
    } else {
        println!("Input not recognized.");
    }
}

fn fib() {
    println!("Fib function!");
}
