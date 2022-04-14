
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GrammaticalValue
{
    Operator { id: u64, value: char },
    NumericalLiteral { id: u64, value: char },
    Identifier { id: u64, value: char },
    Nil { id: u64 }
}
//todo create wrap type
#[derive(Debug)]
pub struct GrammarVec(pub Vec<GrammaticalValue>);

impl GrammarVec 
{
    pub fn into_str(&self) -> String
    {
        let mut vec: Vec<char> = Vec::new();
        let GrammarVec(this) = &*self; 
        for char in this.iter() {
            match char
            {
                GrammaticalValue::Operator { id, value } => { vec.push(*value); },
                GrammaticalValue::NumericalLiteral { id, value } => { vec.push(*value); },
                GrammaticalValue::Identifier { id, value } => { vec.push(*value); },
                _ => { (); } 
            }
        }
        return vec.iter().collect::<String>();
    }
}



pub struct Lexer();

impl Lexer 
{
    /* seperate input by chars and operators */
    #[allow(non_snake_case)]
    pub fn lexInput(input: &str) -> GrammarVec {
        let mut map = Vec::new();
        let mut count = 0;
        input.chars().for_each(
            |x| { // Is it a character or word?
                match x {
                    'a'..='z' => { map.push(GrammaticalValue::Identifier{ id: count, value: x }); count = count + 1; }
                    '0'..='9' => { map.push(GrammaticalValue::NumericalLiteral{ id: count, value: x }); count = count + 1; }
                    ' ' | '\n' => { map.push(GrammaticalValue::Nil{ id: count }); count = count + 1; }
                    _ => { map.push(GrammaticalValue::Operator{ id: count, value: x }); count = count + 1; }
                }
            }
        );
        return GrammarVec(map);
    }

    pub fn collectGrammar(vec: GrammarVec) -> Vec<String>
    {
        let GrammarVec(vec) = vec; 
        let mut result_vec = Vec::new();
        let mut tempvec = Vec::new();
        let (mut start, mut end): (u64, u64) = (0, vec.len() as u64);

        for grammar in vec.iter() {
            match grammar 
            {
                GrammaticalValue::Operator { id, value } => {
                    tempvec = vec[(start as usize)..((*id) as usize)].to_vec();
                    result_vec.push(GrammarVec(tempvec).into_str());
                    start = *id;
                }
                GrammaticalValue::Nil { id } => { 
                    tempvec = vec[(start as usize)..((*id) as usize)].to_vec();
                    result_vec.push(GrammarVec(tempvec).into_str());
                    start = *id;
                }
                _ => { (); }
            }
        }
        return result_vec;
    }
}