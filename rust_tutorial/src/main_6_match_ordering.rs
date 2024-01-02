#![allow(unused)]

// if we want to include everything from io
//use std::io::*;

// Import necessary libraries/modules from the Rust standard library
use std::io; // Input/output functionalities
use rand::Rng; // Random number generation
use std::io::{Write, BufReader, BufRead, ErrorKind}; // More IO functionalities
use std::fs::File; // File handling functionalities
use std::cmp::Ordering; // Comparison functionalities
fn main() {
    println!("###if condition###");
    let age: i32 = 8;
    if(age >= 1) && (age <= 18){
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    // Ternary operator
    println!("###Ternary operator###");
    let mut my_age: i32 = 47;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can Vote : {}", can_vote);

    // Match condition
    println!("###Match###");
    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthaday"),
        65..=i32::MAX => println!("Important Birthday"),
        // match everything else
        _ => println!("Not an Important Birthday"),

    };
    
    println!("### Match 2 and Ordering ###");
    let my_age: i32 = 18;
    let voting_age: i32 = 18;

    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    }

}
