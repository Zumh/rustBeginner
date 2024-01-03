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
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main(){
    pub struct Bank {
        balance: f32
    }
    
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current Balance : {} Withdrawal a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer {} Withdrew {} Current Balance", amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00}));
    
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}

// problem and doesn't work
/*
fn thread_out_live_main(){
    pub struct Bank {
        balance: f32
    }

    fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance -= amt;
    }

    let mut bank = Bank{balance: 100.0};
    withdraw(&mut bank, 5.00);
    println!("Balance : {}", bank.balance);

    fn customer(the_bank: &mut Bank){
        withdraw(the_bank, 5.00);
    }

    thread::spawn(|| {
        customer(&mut bank)

    }).join().unwrap();

}
*/
fn main_spawned_thread() {
    println!("### Concurrency ###");

    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1))
    }
    
    thread1.join().unwrap();

}

