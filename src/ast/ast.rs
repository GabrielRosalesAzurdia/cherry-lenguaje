use crate::lexer::types::TokenType;
use std::fmt;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String; 
    fn a_string(&self) -> String;
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

    fn a_string(&self)->String{
        let mut x = String::from("");

        for i in &self.Statements{
            x = x + &i.a_string();
        }

        x
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

    fn a_string(&self) -> String{
        self.value.to_string()
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

    fn a_string(&self) -> String {
        let mut x = String::from("");
        x = x + &self.token_literal();
        x = x + " ";
        x = x + &self.name.a_string();
        x = x + " = ";

        match &self.value {
            Some(p) => { x = x + &p.a_string(); },
            None => {}
        };

        x = x + ";";

        x
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

    fn a_string(&self) -> String {
        let mut x = String::from("");
        x = x + &self.token_literal();
        x = x + " ";

        match &self.return_value {
            Some(p) => { x = x + &p.a_string(); },
            None => {}
        };

        x = x + ";";

        x
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

#[warn(dead_code)]
pub struct ExpressionStatement {
    pub token : TokenType, // the first token of the expression
    pub expression : Box<dyn Expression> , 
}

impl Node for ExpressionStatement{
    fn token_literal(&self) -> String{
        return self.token.literal.to_string();
    }

    fn a_string(&self) -> String{
        let mut x = String::from("");

        match &self.expression {
            p => { x = x + &p.a_string(); x },
        }

    }
}

impl Statement for ExpressionStatement{
    fn statement_node(&self){
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }    
}

// -----------------------------------------------------------------------------

#[warn(dead_code)]
pub struct IntegerLiteral {
    pub token : TokenType,
    pub value : i32 
}

impl Node for IntegerLiteral{
    fn token_literal(&self) -> String{
        return self.token.literal.to_string();
    }

    fn a_string(&self) -> String{

        return self.token.literal.to_string();

    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self){
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }    
}

//------------------------------------------------------------------------------

#[warn(dead_code)]
pub struct PrefixExpression{
    pub token: TokenType, // The prefic token eg. !
    pub operator : String,
    pub rigth : Box<dyn Expression>
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn a_string(&self) -> String {
        let mut x = String::from("");
        x = x + "(";
        x = x + &self.operator;
        x = x + &self.rigth.a_string();
        x = x + ")";
        x
    }
}

impl Expression for PrefixExpression{
    fn expression_node(&self){

    }

    fn as_any(&self) -> &dyn Any {
        self
    }    
}

//------------------------------------------------------------------------------

#[warn(dead_code)]
pub struct InfixExpression{
    pub token : TokenType,
    pub left : Box<dyn Expression>,
    pub operator : String ,
    pub rigth : Box<dyn Expression>
}

impl Node for InfixExpression{
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn a_string(&self) -> String {
        let mut x = String::from("");
        x = x + "(";
        x = x + &self.left.a_string();
        x = x + " " + &self.operator + " ";
        x = x + &self.rigth.a_string();
        x = x + ")";
        x
    }
}

impl Expression for InfixExpression{
    fn expression_node(&self){

    }

    fn as_any(&self) -> &dyn Any {
        self
    }   
}

//------------------------------------------------------------------------------

#[warn(dead_code)]
#[derive(Debug)]
pub struct Boolean {
    pub token : TokenType,
    pub value : bool
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn a_string(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Expression for Boolean {
    fn expression_node(&self){

    }

    fn as_any(&self) -> &dyn Any {
        self
    } 
}

//------------------------------------------------------------------------------

