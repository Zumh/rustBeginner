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
    println!("### Smart Pointer and Box ###");
    // Stack : FIFO
    // Data on stack define fixed size

    let b_int1 = Box::new(10);

    println!("b_int1 = {}", b_int1);
    
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T, 
    }
    
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {left: None, right: None, key,}
        }
        
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }

    }

    let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3));



}

