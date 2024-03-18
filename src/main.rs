mod compiler;
use compiler::*;
use std::env;

fn main() {
    println!("Hello, world!");
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() <= 1 {
        println!("Enter an input file!");
        return;
    }
    let token_type = Token::Operator(Operator::Addition);
    println!("{:?}", token_type);
}
