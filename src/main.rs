//this calculator will do two things to begin with: calculate nth fibonacci numbers
//  and calculate F to C and C to F

use::std::io;

fn main() {
    
    //prints selection menu
    println!("Select the function you want.\nEnter 1 for Fibonacci number calculator\nEnter 2 for Celsius to Farenheit\nEnter 3 for Farenheit to Celsius");
    
    //initializes an empty string. Only use the empty string literal with an immutable string
    //  variable
    let mut selection = String::new();

    //stdin().read_line() grabs a string from the command line
    io::stdin().read_line(&mut selection).expect("Failed to read line.");
    
    //selection is reassigned to itself, but as type u8
    //in Rust, we say that the previous value/type is shadowed by the new one
    let selection: u8 = selection.trim().parse().expect("");
    
    //selection menu logic
    if selection == 1 {
        fib();
    } else if selection == 2 {
        c_to_f();
    } else if selection == 3 {
        f_to_c();
    } else {
        println!("Input not recognized.");
    }
}

fn fib() {
    println!("Fib function!");
}

fn c_to_f() {
    println!("C to F function!");
}

fn f_to_c() {
    println!("F to C function!");
}
