use std::io;
use std::io::Write;

fn main() {

    let mut y = String::new();

    println!("Enter a number: ");
    io::stdin().read_line(&mut y).expect("Failed to read line");


    let mut y: u32 = y.trim().parse().expect("Please type a number!");
    dbg!(y);

    while y > 1 {
        for _x in 1..11 {
            // println!("{}", _x);
            io::stdout().flush().unwrap();
            print!("#");
        }
        y -= 1;
    }

    // println!("Enter your weight (lbs): ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    //
    // //borrow_string(&input);
    // //own_string(input);
    //
    // //println!("Input: {}", input);
    // let weight: f32  = input.trim().parse().unwrap();
    // let mars_weight: f32 = calculate_weight_on_mars(weight);
    // println!("Weight on Mars: {}", mars_weight);
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    (weight_on_earth / 9.81) * 3.711
}

// pass reference to string
// aka "borrowing" the string by address
fn borrow_string(s: &String) {
    println!("{}", s);
}

// change ownership of string by passing
// the actual string in memory, and not a reference/address, to the fn
fn own_string(s: String) {
    println!("{}", s);
}

/*
    Ownership in Rust
    1. Each value in Rust is owned by a variable
    2. When the owner goes out of scope, the value will be deallocated
    3. There can only be ONE owner at a time

*/