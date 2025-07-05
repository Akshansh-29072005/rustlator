// use std::env::args;

// use std::result;

use core::f64;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, long_about, version)]

struct Args{
    #[arg(short, long)]
    first_number:f64,

    #[arg(short, long)]
    second_number:f64,

    #[arg(short, long)]
    operation:String,
}

fn main(){
    println!("Welcome to Rustlator: A CLI calculator written in Rust!");

    let args = Args::parse();

    let oper = args.operation.to_lowercase();

    if  oper == String::from("add"){
        let result = args.first_number + args.second_number;
        println!("The sum of {} and {} is {}.",args.first_number,args.second_number,result);
    } else if oper == String::from("subtract") {
        let result = args.first_number - args.second_number;
        println!("The difference of {} and {} is {}.",args.first_number,args.second_number,result);
    } else if oper == String::from("multiply") {
        let result = args.first_number * args.second_number;
        println!("The multiplication of {} and {} is {}.",args.first_number,args.second_number,result);
    } else if oper == String::from("divide") {
        let result = args.first_number/args.second_number;
        println!("The division {} and {} is {}.",args.first_number,args.second_number,result);
    } else if oper == String::from("modulus") {
        let result = args.first_number % args.second_number;
        println!("The modulus of {} and {} is {}.",args.first_number,args.second_number,result);
    } else {
        println!("Please enter valid operation.");
    };
    

}