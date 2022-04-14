use crate::frontend::token::*;

pub enum Operation
{
    Multiply(TokenType, TokenType), 
    Add(TokenType, TokenType), 
    Subtract(TokenType, TokenType),
    Divide(TokenType, TokenType)
}

pub struct Calculator
{
    pub left: i32,
    pub right: i32
}

impl Calculator
{
    pub fn calc(&mut self, operation: Operation)
    {
        match operation
        {
            Operation::Multiply(opleft, opright) => {
                match (opleft, opright)
                {
                    (TokenType::Variable(l), TokenType::Variable(r)) => {
                        let opleft = l;
                        let opright = r;
                        (*self).left = (opleft).parse().unwrap();
                        (*self).right = (opright).parse().unwrap();
                        println!("Result: {0}", (self.right * self.left));
                    }
                    _ => { panic!("unsupported"); }
                }
            }, 
            Operation::Add(opleft, opright) => {
                match (opleft, opright)
                {
                    (TokenType::Variable(l), TokenType::Variable(r)) => {
                        let opleft = l;
                        let opright = r;
                        (*self).left = (opleft).parse().unwrap();
                        (*self).right = (opright).parse().unwrap();
                        println!("Result: {0}", (self.right + self.left));
                    }
                    _ => { panic!("unsupported"); }
                }
            }, 
            Operation::Subtract(opleft, opright) => {
                match (opleft, opright)
                {
                    (TokenType::Variable(l), TokenType::Variable(r)) => {
                        let opleft = l;
                        let opright = r;
                        (*self).left = (opleft).parse().unwrap();
                        (*self).right = (opright).parse().unwrap();
                        println!("Result: {0}", (self.right - self.left));
                    }
                    _ => { panic!("unsupported"); }
                }
            },
            Operation::Divide(opleft, opright) => {
                match (opleft, opright)
                {
                    (TokenType::Variable(l), TokenType::Variable(r)) => {
                        let opleft = l;
                        let opright = r;
                        (*self).left = (opleft).parse().unwrap();
                        (*self).right = (opright).parse().unwrap();
                        println!("Result: {0}", (self.right / self.left));
                    }
                    _ => { panic!("unsupported"); }
                }
            }
        }
    }
}