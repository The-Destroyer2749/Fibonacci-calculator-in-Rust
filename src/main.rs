// imports
use text_io::*;
use num::BigUint;

fn main() {
    // const
    let zero_as_vec: Vec<u32> = vec![0];
    let one_as_vec: Vec<u32> = vec![1];
    let big_uint_0: BigUint = BigUint::new(zero_as_vec);
    let big_uint_1: BigUint = BigUint::new(one_as_vec);
    
    // mutable
    let mut running: bool = true;
    let mut input1: i128;
    let mut iteration_goal: i128;
    let mut result: (BigUint, BigUint);
    let mut num1: BigUint = big_uint_0;
    let mut num2: BigUint = big_uint_1;
    let mut print_step_size: i128;
    
    println!("Welcome!\n\n");
    
    while running == true { // program loop
        println!("Input 1 to ask for specific Fibonacci numbers \nInput 2 for a iterative output up to a certain number \nInput 3 to exit the program");
        let program: i8 = read!();
        
        if program == 1 { // direct Fibonacci number program
            println!("What fibonacci number do you want?");
            input1 = read!();
            iteration_goal = input1;
            result = fib(num1.clone(), num2.clone(), 0, iteration_goal);
            println!("The Fibonacci sequence at the index {} is: {}\n", iteration_goal, result.0);
        }
        else if program == 2 { // iterative Fibonacci number program
            println!("What number do you want to iterate up to?");
            iteration_goal = read!();
            println!("How many iterations should it preform before printing each value?");
            print_step_size = read!();
            for i in 0..iteration_goal + 1 {
                result = (num2.clone(), num1 + num2);
                num1 = result.0;
                num2 = result.1;
                if i % print_step_size == 0 || i == iteration_goal {
                    println!("The result at iteration {} is: {}\n", i, num2);
                }
            }
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