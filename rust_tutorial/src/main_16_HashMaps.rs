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
    println!("### HashMaps ###");
    
    let mut heroes = HashMap::new();

    heroes.insert("Superman", "Clark Kent");

    heroes.insert("Batman", "Bruce Wayne");

    heroes.insert("The Flash", "Barry Allen");

    for(k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }

    println!("Length : {}", heroes.len());

    // checking specific key in hash HashMaps
    if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

}
