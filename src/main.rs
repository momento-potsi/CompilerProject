// use std::fs::File;
use std::io;
mod frontend;
use crate::frontend::lexer::Lexer;
use crate::frontend::token::*;
use crate::frontend::executor::*;

/*
    author: Tosin Daudu,
    description: simple working compiler in rust
*/


pub fn main() -> io::Result<()> 
{
    let mut string = String::new();
    io::stdin().read_line(&mut string)?; 
    let result = Token::collectTokens(Lexer::lexInput(string.trim()));
    if result.len() == 6 {
        let op = Operation::Multiply(result[0].clone(), result[4].clone());
        let mut calc = Calculator { left: 0, right: 0 };
        calc.calc(op);
    }
    println!("{:#?}", result);
    Ok(())
}
