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
fn say_hello(){
    println!("Hello");
}

fn get_sum(x: i32, y: i32){
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum2(x: i32, y: i32) -> i32{
    x + y
}

fn get_sum3(x: i32) -> (i32, i32){
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32{
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn main() {
    println!("### Function ###");

    say_hello();
    get_sum(5, 4);
    println!("{}", get_sum2(5, 4));
    
    let (val_1, val_2) = get_sum3(3);

    println!("Nums: {} {}", val_1, val_2);

    let num_list: Vec<i32> = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));
}
