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
    println!("### Array ###");
    
    let arr_1: [i32; 9] = [1,2,3,4, 5, 6, 7, 8, 9];

    println!("1st : {}", arr_1[0]);

    println!("Length : {}", arr_1.len());

    println!("### Loop through index in array ###");
    
    let mut loop_indx: usize = 0;
    loop {
        if arr_1[loop_indx] % 2 == 0 {
            loop_indx += 1;
            continue;
        }
        if arr_1[loop_indx] == 9{
            break;
        }
        println!("Val : {}", arr_1[loop_indx]);
        loop_indx += 1;
    }

    println!("### Array - while loop ###");
    let mut loop_wh_indx: usize = 0;

    while loop_wh_indx < arr_1.len(){
        println!("Arr : {}", arr_1[loop_wh_indx]);
        loop_wh_indx += 1;
    }

    println!("### Array - For loop ###");
    for val in arr_1.iter() {
        println!("Val : {}", val);
    }
}
