use crate::lexer::types::TokenType;
use crate::lexer::types::Lexer;
use crate::lexer::types::Token;
use crate::ast::ast;
use crate::ast::ast::Statement;
use std::collections::HashMap;

// pub enum SIMBOLS1 {
//     LOWEST (u8) ,
//     EQUALS (u8) , // ==
//     LESSGREATHER (u8) , // < OR >
//     SUM (u8) , // +
//     PRODUCT (u8) , // *
//     PREFIX (u8) , // -X OR !X
//     CALL (u8) // MYFUNCTION() 
// }

pub const LOWEST : i32 = 1;
pub const EQUALs : i32 = 1;
pub const LESSGREATHER : i32 = 1;
pub const SUM : i32 = 1;
pub const PRODUCT : i32 = 1;
pub const PREFIX : i32 = 1;
pub const CALL : i32 = 1;


pub enum FUNCIONES {
//    PREFIXPARSEFN( for<'r> fn(&'r mut Parser) -> std::boxed::Box<(dyn ast::Expression + 'static)>  ),
    PREFIXPARSEFN( (for<'r> fn(&'r Parser) -> std::option::Option<std::boxed::Box<(dyn ast::Expression + 'static)>>, String) ),
    PREFIXPARSEFN2( (for<'r> fn(&'r mut Parser) -> std::option::Option<std::boxed::Box<(dyn ast::Expression + 'static)>>, String) ),
    INFIXPARSEFN( fn(e : dyn ast::Expression) ->  Option<Box< dyn ast::Expression>> ),
}

// fn prefix_parse_fn () ->  Option<Box< dyn ast::Expression>> {
//     None
// }

// fn infix_parse_fn ( a : impl ast::Expression ) ->   Option<Box< dyn ast::Expression>> {
//     None
// }

// Ejemplo usando el enum
// let a = FUNCIONES::PREFIXPARSEFN(  prefix_parse_fn ) ;

#[allow(dead_code)]
pub enum ResultadoParseStatement {
    ParseLetStatement(Option<ast::let_statement>),
    ParseResultStatement(Option<ast::ReturnStatement>),
    ParseExpressionStatement1(Option<ast::ExpressionStatement>)
}

pub struct Parser {
    l : Lexer,
    pub errors : Vec<String>,
    cur_token : TokenType,
    peek_token : TokenType,
    prefixParseFns : HashMap<TokenType,FUNCIONES>,
    infixParseFns : HashMap<TokenType,FUNCIONES>,
}

impl Parser{

    pub fn parse_prefix_expression(&mut self) -> Option<Box<dyn ast::Expression >> {

        let token_struct = self.cur_token.clone();
        let operator_struct = self.cur_token.literal.to_string();

        self.next_token();

        let expresion = ast::PrefixExpression{
            token : token_struct,
            operator : operator_struct,
            rigth :  self.parse_expression(PREFIX).unwrap()
        };

        Some(Box::new(expresion))

    }

    pub fn no_prefix_parse_fn_error(&mut self,t : TokenType){
        let msg = format!("no prefix parse function for: {:?}, found", t);
        self.errors.push(msg);
    }

    pub fn parse_integer_literal(&self) -> Option<Box<dyn ast::Expression >> {
        let mut lit = ast::IntegerLiteral{token:self.cur_token.clone(), value:0};
        
        let my_int = self.cur_token.literal.parse::<i32>();
        
        match my_int {
            Ok(a) => { lit.value = a; Some(Box::new(lit)) },
            Err(_) => None
        }

    }

    pub fn parse_identifier(&self) -> Option<Box<dyn ast::Expression>> {
        return Some(Box::new( ast::Identifier{token:self.cur_token.clone(), value: self.cur_token.literal.to_string()} ))
    }

    pub fn parse_expression(&mut self,precedence : i32) -> Option<Box<dyn ast::Expression>> {

        let mut x = self.cur_token.clone();

        x.literal = "".to_string();

        let prefix = self.prefixParseFns.get(&x);

        match prefix {
            Some( FUNCIONES::PREFIXPARSEFN(c) ) => {
                let left_exp = c.0(self); 
                println!("prefixparserfn, el left-exp es {:?}", &left_exp);  
                match left_exp{
                    Some(c) => Some(c), 
                    None => { 
                        let mut err = "".to_string();

                        if c.1 == "parse_integer_literal".to_string(){
                            err = "el caracter dado no es un numero o no es valido".to_string();
                        }

                        if err.len() != 0{
                            self.errors.push(err);
                        }

                        None
                    }
                }
            },

            Some( FUNCIONES::PREFIXPARSEFN2(c) ) => {
                let left_exp = c.0( self ); 
                println!("prefixparserfn2, el left-exp es {:?}", &left_exp);  
                match left_exp{
                    Some(c) => Some(c), 
                    None => None
                }      
            },

            _ => { println!("dio nulo en el prefix"); self.no_prefix_parse_fn_error(self.cur_token.clone()); None}
        }

    }

    pub fn register_prefix(&mut self,t:TokenType, f : FUNCIONES ){

        // match self.prefixParseFns.get(&t){
        //     Some(_) => return,
        //     None => {        
        //                 let a = self.prefixParseFns.get_mut(&t) ;
        //                 *a.unwrap() = f;
        //             }
        // };

        self.prefixParseFns.insert(t, f);

    }   

    pub fn register_Infix(&mut self, t:TokenType, f:FUNCIONES){

        // match self.infixParseFns.get(&t){
        //     Some(_) => return,
        //     None => {        
        //                 let a = self.infixParseFns.get_mut(&t) ;
        //                 *a.unwrap() = f;
        //             }
        // };

        self.infixParseFns.insert(t, f);

    }

    pub fn peek_error(&mut self, t :TokenType){
        let msg = format!("expected next token to be {:?}, got {:?}", t.type_token, self.peek_token.type_token );
        self.errors.push(msg);
    }

    pub fn next_token(&mut self){
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_statement (&mut self) -> Option< ResultadoParseStatement > {

        if self.cur_token.type_token == Token::LET{
            return match self.parse_let_statement() {
                Some(x) => Some(ResultadoParseStatement::ParseLetStatement(Some(x))),
                None => None,
            };
        }
         
        else if self.cur_token.type_token == Token::RETURN{
            return match self.parse_return_statement(){
                Some(r) => Some(ResultadoParseStatement::ParseResultStatement(Some(r))),
                None => None,
            };
        }

        else {
            return match self.parse_expression_statement(){
                Some(c) => Some(ResultadoParseStatement::ParseExpressionStatement1(Some(c))) ,
                None => None
            };
        }

        // #[warn(unreachable_code)]
        // return None;    
    }

    pub fn parse_expression_statement(&mut self) -> Option<ast::ExpressionStatement>{

        let mut stmt = match self.parse_expression(LOWEST){
            Some(c) => ast::ExpressionStatement{token:self.cur_token.clone(), expression: c },
            None => return None
        };

        println!("the stmt exprecion is {:?}", &stmt.expression);
        if self.peek_token_is(Token::SEMICOLON){
            self.next_token();
        }
        Some(stmt)
    }

    pub fn parse_return_statement(&mut self) -> Option<ast::ReturnStatement>{
        let stmt = ast::ReturnStatement{token:self.cur_token.clone() , return_value:None};
        self.next_token();

        //TODO: we are skipping the exprescions until we 
        // Enconunter a semicolon
        while !self.cur_token_is(Token::SEMICOLON){
            self.next_token();
        }

        return Some(stmt);
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::let_statement> {

        let mut stmt = ast::let_statement{
            value : None, 
            token : self.cur_token.clone(), 
            name: ast::Identifier{
                token : TokenType{type_token:Token::EOF,literal:"".to_string()},
                value : "".to_string() 
            }, 
        };

        if !self.expect_peek(Token::IDENT){
            return None;
        }

        stmt.name = ast::Identifier{token : self.cur_token.clone(), value: self.cur_token.literal.to_string()};

        if !self.expect_peek(Token::ASSING){
            return None
        } 

        // TODO: We are Skiping the exprecion until we encounter a semicolon
        while self.cur_token_is(Token::SEMICOLON){
            self.next_token();
        }

        return Some(stmt);

    }

    pub fn cur_token_is (&self, token : Token) -> bool { 
        return self.peek_token.type_token == token;
    } 

    pub fn peek_token_is (&self, token : Token) -> bool {
        return self.peek_token.type_token == token;
    }

    pub fn expect_peek(&mut self, t : Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        }
        else{
            let struct_to_error = TokenType{type_token: t.clone(), literal:"".to_string()}; 
            self.peek_error(struct_to_error);
            return false;
        }
    } 

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program{ Statements:vec![] };

        while self.cur_token.type_token != Token::EOF{
            let mut stmt = self.parse_statement();

            match stmt {    
                Some(_) => {
                    let x = stmt.unwrap();
                    match x {
                        ResultadoParseStatement::ParseLetStatement(c) => { let resultado = c.unwrap();  program.Statements.push(Box::from(resultado)) ;},
                        ResultadoParseStatement::ParseResultStatement(c) => { let resultado = c.unwrap();  program.Statements.push(Box::from(resultado)) ;},
                        ResultadoParseStatement::ParseExpressionStatement1(c) => { let resultado = c.unwrap();  program.Statements.push(Box::from(resultado)) ;},
                    };
                },
                None => println!("dio nulo el parse program") 

            };

            self.next_token();
        }

        return program;

    }

}

pub fn new(l : &Lexer) -> Parser{
    let le = l.clone();
    let mut p = Parser{ 
        l : le, 
        errors : vec![],
        cur_token : TokenType{ type_token:Token::EOF ,literal:"".to_string()},  
        peek_token: TokenType{ type_token:Token::EOF ,literal:"".to_string()},
        prefixParseFns : HashMap::new(),
        infixParseFns : HashMap::new()
    };

    p.register_prefix(TokenType{type_token:Token::IDENT,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN( (Parser::parse_identifier, "parse_identifier".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::INT,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN( (Parser::parse_integer_literal, "parse_integer_literal".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::BANG,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_prefix_expression, "parse_prefix_expression".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::MINUS,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_prefix_expression, "parse_prefix_expression".to_string()) ));

    p.next_token();
    p.next_token();

    return p;

}


