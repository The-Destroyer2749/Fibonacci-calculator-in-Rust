use text_io::*;
use num::BigUint;

fn main() {
    let mut useless_string: String;
    let mut input1: i128;
    let mut input2: String;
    let mut iteration_goal: i128;
    let mut result: (BigUint, BigUint);
    let zero_as_vec: Vec<u32> = vec![0];
    let one_as_vec: Vec<u32> = vec![1];
    let big_uint_0: BigUint = BigUint::new(zero_as_vec);
    let big_uint_1: BigUint = BigUint::new(one_as_vec);
    let mut num1: BigUint = big_uint_0;
    let mut num2: BigUint = big_uint_1;
    let mut running: bool = true;
    
    println!("Welcome!\n\n");
    while running == true {
        println!("Input 1 to ask for specific fibonacci numbers \ninput 2 for a iterative output up to a certain number \n input 3 to exit the program");
        let program: i8 = read!();

        if program == 1 {
            loop {
                println!("What fibonacci number do you want?");
                input1 = read!();
                iteration_goal = input1;
                result = fib(num1.clone(), num2.clone(), 0, iteration_goal);
                println!("The fibonacci sequence at the index: {} is {}", iteration_goal, result.0);
                println!("\nWould you like to continue?");
                input2 = read!();
                input2 = input2.to_lowercase();
                if input2 != "true" && input2 != "yes" && input2 != "y" {
                    break;
                }
            }
        } else if program == 2 {
            println!("what number do you want to iterate up to?");
            iteration_goal = read!();
            for i in 0..iteration_goal + 1 {
                result = (num2.clone(), num1 + num2);
                num1 = result.0;
                num2 = result.1;
                println!("The result at iteration: {} is {}", i, num2);
            }
        }
        else if program == 3 { 
            running = false;
        }
        else { 
            println!("Error: invalid input");
        }
    }
    useless_string = "".to_string();
    println!("This is just to hold off the program :) type anything to end it {}", useless_string);
    useless_string = read!();
    println!("{}", useless_string);
}


fn fib(num1: BigUint, num2: BigUint, mut i: i128, iteration_goal: i128) -> (BigUint,BigUint) {
    i+=1;
    if i == iteration_goal {
        (num1, num2)
    }
    else {
        fib(num2.clone(), num1 + num2, i, iteration_goal)
    }
}