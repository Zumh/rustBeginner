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
    println!("### Iterators ###");
    
    let mut arr_it: [i32; 4] = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
}

