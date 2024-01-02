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

// Key value pair
fn main() {
    println!("### struct ###");

    struct Customer {
        name: String,
        address: String,
        balance: f32, 
    }

    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50
    };
    println!("name: {} \naddress: {} \nown: {}", bob.name, bob.address, bob.balance);
    bob.address = String::from("505 Main St");

    
    normal_struct();


}

fn normal_struct(){
    struct Rectangle<T, U> {
        length: T,
        height: U,

    }

    let rec = Rectangle {
        length: 4, height: 10.5
    };

}
