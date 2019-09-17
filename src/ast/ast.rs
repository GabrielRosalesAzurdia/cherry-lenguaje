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

#[warn(dead_code)]
pub struct IfExpression {
    pub token : TokenType,
    pub condition : Box<dyn Expression>,
    pub consequence : Option<BlockStatement>,
    pub alternative : Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn a_string(&self) -> String {
        let mut x = String::from("if");
        x = x + &self.condition.a_string();
        x = x + " ";

        match &self.consequence {
            Some(c) => x = x + &c.a_string(),
            None => x = x + "no hay consequence", 
        };

        match &self.alternative {
            Some(c) => {x=x+"else ";  x = x + &c.a_string();},
            None => x = x + "no hay alternative", 
        };

        x
    }
}

impl Expression for IfExpression {
    fn expression_node(&self){

    }

    fn as_any(&self) -> &dyn Any {
        self
    } 
}

//------------------------------------------------------------------------------

#[warn(dead_code)]
pub struct BlockStatement{
    pub token : TokenType,
    pub statements : Vec<Box<dyn Statement>>
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn a_string(&self) -> String {

        let mut x = "".to_string();

        for i in self.statements.iter(){
            x = x + &i.a_string();
        }

        x
    }
}

impl Statement for BlockStatement {
    fn statement_node (&self){

    }

    fn as_any(&self) -> &dyn Any {
        self
    } 
}

//------------------------------------------------------------------------------

#[warn(dead_code)]
pub struct FunctionLiteral {
    pub token : TokenType,
    pub parameters : Vec<Identifier>,
    pub body : BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn a_string(&self) -> String {
        let mut x = String::from("");
        let mut params:Vec<String> = vec![];

        // let mut a = &vec![];

        // match self.parameters.as_ref() {
        //     Some(c) => a = c,
        //     None => (),
        // };

        for i in self.parameters.iter() {
            params.push(i.a_string());
        }

        x = x + &self.token_literal();
        x = x + "(";

        for i in params {
            x = x + &i + ", ";
        }

        x = x + ")";
        x = x + &self.body.a_string();
        x
    }
}

impl Expression for FunctionLiteral {
    fn expression_node (&self){

    }

    fn as_any(&self) -> &dyn Any {
        self
    } 
}

//------------------------------------------------------------------------------

#[warn(dead_code)]
pub struct CallExpression{
    pub token : TokenType ,
    pub function : Box<dyn Expression>,
    pub arguments : Vec<Box<dyn Expression>>
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn a_string(&self) -> String {
        let mut x = String::from("");
        let mut args : Vec<String> = vec![];

        for i in self.arguments.iter() {
            args.push(i.a_string());
        }

        x = x + &self.function.a_string();
        
        x = x + "(";

        let mut c = 0;

        for i in args.iter() {
            c = c + 1 ;
            println!("vamos a evaluar a i: {}, {} , {}", i, c , args.len());
            if args.len() != 1 && c < args.len(){
                x = x + i + ", ";
            }
            else{
                x = x + i;
            }
        }

        x = x + ")";

        x
    }
}

impl Expression for CallExpression {
    fn expression_node (&self){

    }

    fn as_any(&self) -> &dyn Any {
        self
    } 
}