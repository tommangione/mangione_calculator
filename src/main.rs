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
    println!("\nFibonacci generator\n\nEnter the desired index as a positive integer greater than 0.\n");
    //logic: Fibonacci number n at index i equals n[i-1] + n[i-1] ...
    
    let mut fib_index = String::new();
    let mut value: usize;

    io::stdin().read_line(& mut fib_index).expect("Failed to read line.");

    let fib_index: usize = fib_index.trim().parse().expect("");
    let mut fib_vector = vec![0, 1];

    if fib_index < 1 {
        panic!("Error: index not recognized");
    } else if fib_index > fib_vector.len() {
        while fib_index > fib_vector.len() {
            value = fib_vector[fib_vector.len()-1] + fib_vector[fib_vector.len()-2];
            fib_vector.push(value);
        }
        println!("\n{}", fib_vector[fib_index-1]);
    } else {
        println!("\n{}", fib_vector[fib_index-1]);
    }
}

fn c_to_f() {
    println!("C to F function!");
}

fn f_to_c() {
    println!("F to C function!");
}
