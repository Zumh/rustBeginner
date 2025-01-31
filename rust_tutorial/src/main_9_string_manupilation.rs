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
    println!("### String ###");

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    println!("### Normal string ###\n");

    let st3 = String::from("x r t b h k k a m c");

    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    println!("### Random string ###\n");
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1: &[u8] = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());
    st5.clear();
    
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    println!("### String concatenated ###");
    for char in st8.bytes(){
        println!("{}", char);
    }
}
