// imports
use ::text_io::*;
use ::num::BigUint;
use std::time::{Duration, Instant};

fn main() {
    // const
    let zero_as_vec: Vec<u32> = vec![0];
    let one_as_vec: Vec<u32> = vec![1];
    let big_uint_0: BigUint = BigUint::new(zero_as_vec);
    let big_uint_1: BigUint = BigUint::new(one_as_vec);
    const MILLIS_TO_HRS: u128 = 1000 * 60 * 60;
    const MILLIS_TO_MINS: u128 = 1000 * 60;
    const MILLIS_TO_SECS: u128 = 1000;
    
    // mutable
    let mut running: bool = true;
    let mut input1: i128;
    let mut iteration_goal: i128;
    let mut result: (BigUint, BigUint);
    let mut num1: BigUint = big_uint_0;
    let mut num2: BigUint = big_uint_1;
    let mut print_step_size: i128;
    let mut _time_start: Instant = Instant::now();
    let mut _time_end: Instant = Instant::now();
    let mut _time_delta: Duration = _time_end - _time_start;
    let mut time_benchmark: bool = false;
    let temp_string: String;

    macro_rules! time_benchmark_start {
        () => {
            if time_benchmark == true {
                _time_start = Instant::now();
            }
        };
    }

    macro_rules! time_benchmark_end {
        () => {
            if time_benchmark == true {
                _time_end = Instant::now();
                _time_delta = _time_end - _time_start;
            }
        };
    }

    macro_rules! print_time_benchmark_results {
        () => {
            if time_benchmark == true {
                let mut remaining_time = _time_delta.as_millis();
                let hrs: u128 = remaining_time / MILLIS_TO_HRS;
                remaining_time = remaining_time % MILLIS_TO_HRS;
                
                let mins: u128 = remaining_time / MILLIS_TO_MINS;
                remaining_time = remaining_time % MILLIS_TO_MINS;
                
                let secs: u128 = remaining_time / MILLIS_TO_SECS;
                remaining_time = remaining_time % MILLIS_TO_SECS;
                
                let millis: u128 = remaining_time;

                print!("This operation took ");
                if hrs >= 1 {
                    print!("{} hours ", hrs);
                }
                if mins >= 1 {
                    print!("{} minutes ", mins);
                }
                if secs >= 1 {
                    print!("{} seconds ", secs);
                }
                println!("{} milliseconds", millis);
            }
        };
    }
    
    println!("Welcome!\n\n");

    println!("Type y/n if you would like the calculations to be timed from now on");
    temp_string = read!();
    if temp_string.to_lowercase() == "y" {
        time_benchmark = true;
    }
    while running == true { // program loop
        println!("Input 1 to ask for specific Fibonacci numbers \nInput 2 for a iterative output up to a certain number \nInput 3 to exit the program");
        let program: i8 = read!();


        if program == 1 { // direct Fibonacci number program
            println!("What fibonacci number do you want?");
            input1 = read!();
            iteration_goal = input1;
            time_benchmark_start!();
            result = fib(num1.clone(), num2.clone(), 0, iteration_goal);
            time_benchmark_end!();
            println!("The Fibonacci sequence at the index {} is: {}\n", iteration_goal, result.0);
            print_time_benchmark_results!();
        }
        else if program == 2 { // iterative Fibonacci number program
            println!("What number do you want to iterate up to?");
            iteration_goal = read!();
            println!("How many iterations should it preform before printing each value?");
            print_step_size = read!();
            time_benchmark_start!();
            for i in 0..iteration_goal + 1 {
                result = (num2.clone(), num1 + num2);
                num1 = result.0;
                num2 = result.1;
                if i % print_step_size == 0 || i == iteration_goal {
                    println!("The result at iteration {} is: {}\n", i, num2);
                }
            }
            time_benchmark_end!();
            print_time_benchmark_results!();
        }
        else if program == 3 { // program exit
            running = false;
        }
        else { 
            println!("Error: invalid input\n");
        }
    }
}

fn fib(mut num1: BigUint, mut num2: BigUint, mut i: i128, iteration_goal: i128) -> (BigUint,BigUint) { // iterative Fibonacci solver
    let mut temp: BigUint;
    while i != iteration_goal {
        i+=1;
        temp = num1;
        num1 = num2.clone();
        num2 = num2 + temp;
    }
    (num1, num2)
}