
pub mod lexer;
pub mod token;
pub mod executor;

/*
    Ok so you  have a collection of chars and so what you can do is store the starting count 
    and then while this char isn’t a nil or operator (other than _) char.next but if it is
    then store the end count and then collect start to end count into a str and repeat until end.
    Next create matching and precedence(leftop, rightop, operand, operation) for lex
    Children nodes(bracket(item) operators have children)
    Todo rework token.rs for str and eliminate grammar type shave down but make debug look good 
*/