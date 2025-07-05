use std::io::{self};

enum Operations{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus
}

fn calculator(oper:Operations, first_number:u128, second_number:u128){
    match oper {
        Operations::Addition => {
            println!("The sum of {} and {} is {}",first_number,second_number,first_number+second_number)
        },

        Operations::Subtraction => {
            println!("The difference of {} and {} is {}",first_number,second_number,first_number-second_number)
        },

        Operations::Multiplication => {
            println!("The multiplication of {} and {} is {}",first_number,second_number,first_number*second_number)
        },

        Operations::Modulus => { if second_number !=0 {
            println!("The modulus of {} and {} is {}",first_number,second_number,first_number%second_number)
        } else {
            println!("The modulus of any number with divisor zero is Undefined!");
        };
        },

        Operations::Division => { if second_number !=0{
            println!("The division of {} and {} is {}",first_number,second_number,first_number/second_number);
        } else {
            println!("Cannot divide a number with Zero!");
        }
    }
    }
}

fn main(){
    println!("Welcome to Rustlator: A CLI calculator written in Rust!");

    //Taking the input of first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let first_number:u128 = match input1.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    //Taking the input of second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let second_number:u128 = match input2.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    //Taking the input of the operation
    println!("Enter the operation (add, subtract, multiply, divide, modulus):");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).unwrap();
    let op = input3.trim().to_lowercase();

    let oper = match op.as_str(){
        "add" => Operations::Addition,
        "subtract" => Operations::Subtraction,
        "divide" => Operations::Division,
        "multiply" => Operations::Multiplication,
        "modulus" => Operations::Modulus,
        _ => {
            println!("Please enter a valid operation!");
            return;
        }
    };
    calculator(oper, first_number, second_number);
}