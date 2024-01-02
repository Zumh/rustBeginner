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
    println!("### READING WRITING ERROR HANDLING  ###");
/*
    let lil_arr = [1,2];
    println!("{}", lilarr[10]);
*/
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {panic!("Problem creating file : {:?}", error);}
    };

    // handling error
    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();

    let buffered = BufReader::new(input);

    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }

    // catch specific errror
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create
                ("rand.txt"){
                    Ok(fc) => fc,
                    Err(e) => panic!("Can't create file: {:?}", error),
                },

                _other_error => panic!("Problem opening file : {:?}", error),
                
        },
    };
}

