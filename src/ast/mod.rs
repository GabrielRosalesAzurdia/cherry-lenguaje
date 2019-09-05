use crate::lexer::types::TokenType;
use std::fmt;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String; 
}

pub trait Statement : Node {
    fn statement_node(&self); 
    // fn clonar (&self) -> let_statement;
    fn as_any(&self) -> &dyn Any;
}

impl fmt::Debug for dyn Statement { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_literal() )
    }
}

pub trait Expression : Node {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
}

impl fmt::Debug for dyn Expression { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_literal() )
    }
}

// -----------------------------------------------------------------------------

#[warn(dead_code)]
pub struct Program {
    pub Statements : Vec<Box<dyn Statement>>,
}

impl Node for Program{
    fn token_literal(&self) -> String {
        if self.Statements.len() > 0 {
            return self.Statements[0].token_literal();
        }
        else {
            return "".to_string();  
        }
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Identifier {
    pub token : TokenType, // the token::ident
    pub value : String,
}

impl Node for Identifier{
    fn token_literal(&self) -> String{
        return self.token.literal.to_string();
    }
}

impl Expression for Identifier{
    fn expression_node(&self){
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

// -----------------------------------------------------------------------------

#[warn(dead_code)]
pub struct let_statement {
    pub token : TokenType, // the token::let
    pub name : Identifier,
    pub value : Option<Box<dyn Expression>>,    
}

impl Node for let_statement{
    fn token_literal(&self) -> String{
        return self.token.literal.to_string();
    }
}

impl Statement for let_statement{
    fn statement_node(&self){
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

// -----------------------------------------------------------------------------

#[warn(dead_code)]
pub struct ReturnStatement {
    pub token : TokenType,
    pub return_value :  Option<Box<dyn Expression>>
}

impl Node for ReturnStatement{
    fn token_literal(&self) -> String{
        return self.token.literal.to_string();
    }
}

impl Statement for ReturnStatement{
    fn statement_node(&self){
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }    
}

// -----------------------------------------------------------------------------