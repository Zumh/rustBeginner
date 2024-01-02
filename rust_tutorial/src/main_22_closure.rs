#![allow(unused)]

// if we want to include everything from io
//use std::io::*;

// Import necessary libraries/modules from the Rust standard library
use std::io; // Input/output functionalities
use rand::Rng; // Random number generation
use std::io::{Write, BufReader, BufRead, ErrorKind}; // More IO functionalities
use std::fs::File; // File handling functionalities
use std::cmp::Ordering; // Comparison functionalities
use std::collections::HashMap;


fn main() {
    println!("### Closure ###");
    
    // let var_name = |parameters| -> return type {BODY}
    
    let can_vote = |age: i32| {
        age >= 18 
    };

    println!("Can vote : {}", can_vote(8));

    let mut sample1: i32 = 5;

    let print_var = || println!("samp1 = {}", sample1);

    print_var();

    sample1 = 10;

    let mut change_var = || sample1 += 1;
    change_var();
    println!("samp1 = {}", sample1);
    sample1 = 10;
    println!("samp1 = {}", sample1);

    // pass function to Closure
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
            func(a, b)
    }

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    
    println!("5 = 4 = {}", use_func(5, 4, sum));
    println!("5 + 4 = {}", use_func( 5, 4, prod));
}

