#![allow(unused)]

// if we want to include everything from io
//use std::io::*;

// Import necessary libraries/modules from the Rust standard library
use std::io; // Input/output functionalities
use rand::Rng; // Random number generation
use std::io::{Write, BufReader, BufRead, ErrorKind}; // More IO functionalities
use std::fs::File; // File handling functionalities
use std::cmp::Ordering; // Comparison functionalities
                        //
// memory going to menage base on Ownership
fn print_str(x: String){
    println!("A string {}", x);
}

fn print_return_str(x: String)-> String{
    println!("A string {}", x);
    x
}

fn change_str(name: &mut String){
    name.push_str(" is happy");
    println!("Message : {}", name);
}
fn main() {
    println!("### Ownership ###");
    
    let str1 = String::from("world");
    let str2 = str1.clone(); // copy str1 to str2 
    print_str(str2);  
    println!("Hello {}", str1); // we cannot print this with out clone
    let str3 = print_return_str(str1);                                                
    println!("str3 = {}", str3);
    let mut mut_str = String::from("Derek");
    change_str(&mut mut_str);

}
