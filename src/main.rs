//this calculator will do two things to begin with: calculate nth fibonacci numbers
//  and calculate F to C and C to F

use::std::io;

fn main() {
    
    //prints selection menu
    println!("Select the function you want.\nEnter 1 for Fibonacci number calculator\nEnter 2 for Celsius to Fahrenheit\nEnter 3 for Fahrenheit to Celsius");
    
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
    
    //Intro text
    println!("\nFibonacci generator\n\nEnter the desired index (indexed starting at 1) as a positive integer greater than 0.\n");
    
    //creates input variable and value variable that grows the vector that holds the sequence
    let mut fib_index = String::new();
    let mut value: usize;
    
    //takes input
    io::stdin().read_line(& mut fib_index).expect("Failed to read line.");
    
    //makes fib_index variable into type usize
    let fib_index: usize = fib_index.trim().parse().expect("");
    
    //intializes fib_vector with first two terms of sequence
    let mut fib_vector = vec![0, 1];
    
    //exception handling
    if fib_index < 1 {
        panic!("Error: index not recognized");
    
    //algorithm that grows the vector to the appropriate size before outputting value
    } else if fib_index > fib_vector.len() {
        while fib_index > fib_vector.len() {
            value = fib_vector[fib_vector.len()-1] + fib_vector[fib_vector.len()-2];
            fib_vector.push(value);
        }
        println!("\nTerm {} of the Fibonacci sequence is {}", fib_index, fib_vector[fib_index-1]);
    } else {
        println!("\nTerm {} of the Fibonacci sequence is {}", fib_index, fib_vector[fib_index-1]);
    }
}

fn c_to_f() {
    
    //Intro text
    println!("\nCelsius to Fahrenheit converter\n\nEnter the temperature in Celsius as an integer.");

    //creates input variable
    let mut celsius_temp = String::new();

    //takes input
    io::stdin().read_line(& mut celsius_temp).expect("Failed to read line.");

    //makes calsius_temp into type i32
    let celsius_temp: i32 = celsius_temp.trim().parse().expect("");
    
    //conversion
    let f_output = (celsius_temp * 9/5) + 32;

    println!("{} Celsius is equivalent to {} Fahrenheit.", celsius_temp, f_output);
}

fn f_to_c() {
    println!("F to C function!");
}
