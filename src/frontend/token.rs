use crate::frontend::lexer::GrammaticalValue;
use crate::frontend::lexer::GrammarVec;

#[derive(Clone, Debug)]
pub enum TokenType
{
    Variable(String), 
    Literal(String),
    Operator(String), 
    Keyword(String)
}

#[derive(Debug)]
pub struct Token
{
    pub grammar: GrammarVec,
    id_type: TokenType,
    line_number: Option<u32>
}

impl Token
{
    pub fn collectTokens(input: GrammarVec) -> Vec<TokenType>
    {
        let input = input.0;
        let mut tokens: Vec<char> = Vec::new();
        let mut vec = Vec::new();

        for element in input.iter()
        {
            match element 
            {
                GrammaticalValue::Operator { id, value } => { 
                    vec.push(TokenType::Variable(tokens.iter().collect()));  
                    tokens.clear();
                    vec.push(TokenType::Operator(String::from(value.to_string())));
                },
                GrammaticalValue::NumericalLiteral { id, value } => { tokens.push(*value); },
                GrammaticalValue::Identifier { id, value } => { tokens.push(*value); },
                _ => { 
                    vec.push(TokenType::Variable(tokens.iter().collect::<String>()));  
                    tokens.clear();
                }
            }
        }

        return vec;
    }

    pub fn new(grammar: GrammarVec, tokentype: TokenType) -> Self
    {
        Token {
            grammar: grammar,
            id_type: tokentype,
            line_number: None 
        }
    }
}
/* todo rework */