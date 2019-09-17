use crate::lexer::types::TokenType;
use crate::lexer::types::Lexer;
use crate::lexer::types::Token;
use crate::ab_st_tr::ast;
use std::collections::HashMap;

pub const LOWEST : i32 = 1; // LOWEST VALUE
pub const EQUALS : i32 = 2;  // ==
pub const LESSGREATHER : i32 = 3; // < OR >
pub const SUM : i32 = 4; // *
pub const PRODUCT : i32 = 5;
pub const PREFIX : i32 = 6; // -X OR !X
pub const CALL : i32 = 7; // MYFUNCTION() 

// static mut precedence_var :HashMap<Token,i32> = HashMap::new();

pub fn precedence_var () -> HashMap<Token,i32> {
    let mut x = HashMap::new();
    x.insert(Token::EQUAL, EQUALS);
    x.insert(Token::NOTEQUAL, EQUALS);
    x.insert(Token::LESSTHAN, LESSGREATHER);
    x.insert(Token::BIGERTHAN, LESSGREATHER);
    x.insert(Token::PLUS, SUM);
    x.insert(Token::MINUS, SUM);
    x.insert(Token::SLASH, PRODUCT);
    x.insert(Token::ASTERISK, PRODUCT);
    x.insert(Token::LPAREN, CALL);
    x
}


pub enum FUNCIONES {
//  PREFIXPARSEFN( for<'r> fn(&'r mut Parser) -> std::boxed::Box<(dyn ast::Expression + 'static)>  ),
//  INFIXPARSEFN( fn(e : dyn ast::Expression) ->  Option<Box< dyn ast::Expression>> ),
//  PREFIXPARSEFN( (for<'r> fn(&'r Parser) -> std::option::Option<std::boxed::Box<(dyn ast::Expression + 'static)>>, String) ),
    PREFIXPARSEFN2( (for<'r> fn(&'r mut Parser) -> std::option::Option<std::boxed::Box<(dyn ast::Expression + 'static)>>, String) ),
    INFIXPARSEFN( (for<'r> fn(&'r mut Parser, std::boxed::Box<(dyn ast::Expression + 'static)>) -> std::option::Option<std::boxed::Box<(dyn ast::Expression + 'static)>>, std::string::String) )
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

    pub fn parse_call_arguments(&mut self) ->  Vec<Box<dyn ast::Expression>> {
        self.next_token();
        let mut args = vec![];

        if self.peek_token_is(Token::RPAREN) {
            self.next_token();
            return args;
        } 

        self.next_token();

        // println!("en el punto de inflexion el token es: {:?}", self.cur_token.clone());
        // args.push(self.parse_expression(LOWEST).unwrap());
        match self.parse_expression(LOWEST){
            Some(b) => args.push(b),
            None => ()
        };

        while self.peek_token_is(Token::COMMA){
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(LOWEST).unwrap());
        }

        if !self.expect_peek(Token::RPAREN){
            return vec![];
        }

        // println!("logro enviar los args");
        return args;

    }

    pub fn parse_call_expression(&mut self, function : Box<dyn ast::Expression> ) -> Option<Box<dyn ast::Expression>> {
        let mut i = ast::CallExpression{token : self.cur_token.clone(), function : function, arguments : vec![]};
        i.arguments = self.parse_call_arguments();
        return Some(Box::new(i));
    }

    pub fn parse_function_parameters(&mut self) -> Vec<ast::Identifier> {
        let mut identifiers = vec![];

        if self.peek_token_is(Token::RPAREN){
            self.next_token();
            return identifiers;
        }

        self.next_token();

        let ident = ast::Identifier{token: self.cur_token.clone(), value: self.cur_token.literal.to_string()};
        identifiers.push(ident);

        while self.peek_token_is(Token::COMMA){
            self.next_token();
            self.next_token();
            let ident = ast::Identifier{token: self.cur_token.clone(), value: self.cur_token.literal.to_string()};
            identifiers.push(ident);
        }

        if !self.expect_peek(Token::RPAREN){
            return vec![]; 
        }

        return identifiers;

    }

    pub fn parse_function_literal(&mut self) ->  Option<Box<dyn ast::Expression>> {
        let token_var = self.cur_token.clone();

        if !self.expect_peek(Token::LPAREN){
            return None;
        }

        let parameters_var = self.parse_function_parameters();

        if !self.expect_peek(Token::LBRACE){
            return None;
        }

        let lit = ast::FunctionLiteral {
            parameters : parameters_var,
            token : token_var,
            body  : self.parse_block_statement().unwrap(), 
        };

        return Some(Box::new(lit));
    }

    pub fn parse_block_statement(&mut self) -> Option<ast::BlockStatement> {
        let mut block = ast::BlockStatement{
            statements : vec![],
            token : self.cur_token.clone()
        };

        self.next_token();

        while !self.cur_token_is(Token::RBRACE) && !self.cur_token_is(Token::EOF){

            let stmt = self.parse_statement();

            match stmt {    
                Some(_) => {
                    let x = stmt.unwrap();
                    match x {
                        ResultadoParseStatement::ParseLetStatement(c) => { let resultado = c.unwrap();  block.statements.push(Box::new(resultado)) ;},
                        ResultadoParseStatement::ParseResultStatement(c) => { let resultado = c.unwrap();  block.statements.push(Box::new(resultado)) ;},
                        ResultadoParseStatement::ParseExpressionStatement1(c) => { let resultado = c.unwrap();  block.statements.push(Box::new(resultado)) ;},
                    };
                },
                None => return None
            };

            self.next_token();

        }

        return Some(block);

    }

    pub fn parse_if_expression(&mut self) -> Option<Box<dyn ast::Expression>> {

        let token_if = self.cur_token.clone();

        if !self.expect_peek(Token::LPAREN){
            return None;
        }

        self.next_token();

        let condition_if = self.parse_expression(LOWEST);

        if !self.expect_peek(Token::RPAREN){
            return None;
        }

        if !self.expect_peek(Token::LBRACE){
            return None;
        }

        let block_if = self.parse_block_statement();
        let mut block_if_2 = None;
            
        if self.peek_token_is(Token::ELSE){
            self.next_token();
            if !self.expect_peek(Token::LBRACE){
                return None;
            }
            block_if_2 = self.parse_block_statement();
        }

        return Some(
            Box::new(
                ast::IfExpression{
                    token:token_if,
                    condition:condition_if.unwrap(),
                    consequence : block_if,
                    alternative : block_if_2
                } 
            )
        );

    }

    pub fn parse_grouped_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        self.next_token();

        let exp = self.parse_expression(LOWEST);

        if !self.expect_peek(Token::RPAREN){
            return None;
        }

        return exp;
    }

    pub fn parse_boolean(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::Boolean{ token:self.cur_token.clone(), value:self.cur_token_is( Token::TRUE ) }))
    }

    pub fn parse_infix_expression(&mut self, left: Box<dyn ast::Expression>) -> Option<Box<dyn ast::Expression>> {
        self.next_token();

        let token_var = self.cur_token.clone();
        let operator_var = self.cur_token.literal.to_string();
        let left_var = left;

        let precedence = self.cur_precedence();
        self.next_token();
        
        Some(
            Box::new(
                ast::InfixExpression{
                    token: token_var,
                    operator: operator_var,
                    left: left_var,
                    rigth: self.parse_expression(precedence).unwrap()
                }
            )
        )
    }

    pub fn peek_precedence (&self) -> i32 {
        let pv = precedence_var();
        let result = pv.get(&self.peek_token.type_token);
        match result {
            Some(c) => *c,
            None => LOWEST
        }
    }

    pub fn cur_precedence (&self) -> i32 {
        let pv = precedence_var();
        let result = pv.get(&self.cur_token.type_token);
        match result {
            Some(c) => *c,
            None => LOWEST
        }
    }

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

    pub fn parse_integer_literal(&mut self) -> Option<Box<dyn ast::Expression >> {
        let mut lit = ast::IntegerLiteral{token:self.cur_token.clone(), value:0};
        
        let my_int = self.cur_token.literal.parse::<i32>();
        
        match my_int {
            Ok(a) => { lit.value = a; Some(Box::new(lit)) },
            Err(_) => { 
                // println!("error al parcear el numero"); 
                None
            }
        }

    }

    pub fn parse_identifier(&mut self) -> Option<Box<dyn ast::Expression>> {
        return Some(Box::new( ast::Identifier{token:self.cur_token.clone(), value: self.cur_token.literal.to_string()} ))
    }

    pub fn parse_expression(&mut self,precedence : i32) -> Option<Box<dyn ast::Expression>> {

        let mut x = self.cur_token.clone();
        x.literal = "".to_string();
        let prefix = self.prefixParseFns.get(&x);
        // println!("x es : {:?}",&x);

        match prefix {
            // Some( FUNCIONES::PREFIXPARSEFN(c) ) => {
            //     let left_exp = c.0(self); 
            //     println!("prefixparserfn, el left-exp es {:?}", &left_exp);  
            //     match left_exp{
            //         Some(c) => {

            //             let mut ce = c;

            //             println!("precedence: {}, peek_precedence: {}",precedence, self.peek_precedence()  );

            //             while !self.peek_token_is(Token::SEMICOLON) && precedence < self.peek_precedence(){
            //                 println!("entra al bucle");
            //                 let mut y = self.peek_token.clone();
            //                 y.literal = "".to_string();
            //                 let infix = self.infixParseFns.get(&y);
            //                 match infix {
            //                     Some( FUNCIONES::INFIXPARSEFN(tupla_infix) ) => {

            //                         ce = tupla_infix.0(self,ce).unwrap();

            //                     },
            //                     _ => return Some(ce)
            //                 };

            //             }

            //             Some(ce)
            //         }, 
            //         None => { 
            //             let mut err = "".to_string();

            //             if c.1 == "parse_integer_literal".to_string(){
            //                 err = "el caracter dado no es un numero o no es valido".to_string();
            //             }

            //             if err.len() != 0{
            //                 self.errors.push(err);
            //             }

            //             None
            //         }
            //     }
            // },

            Some( FUNCIONES::PREFIXPARSEFN2(c) ) => {
                // println!("vamos a evaluar a: {}", c.1);
                let left_exp = c.0( self ); 
                // println!("prefixparserfn2, el left-exp es {:?}", &left_exp);  
                match left_exp{
                    Some(c) => {

                        let mut ce = c;

                        // println!("p: {}, x: {}",precedence, self.peek_precedence()  );

                        while !self.peek_token_is(Token::SEMICOLON) && precedence < self.peek_precedence(){

                            // println!("entra al bucle");
                            let mut y = self.peek_token.clone();
                            y.literal = "".to_string();
                            let infix = self.infixParseFns.get(&y);
                            // println!("y es : {:?}",&y);

                            match infix {
                                Some( FUNCIONES::INFIXPARSEFN(tupla_infix) ) => {
                                    // println!("se esta ejecutando un infix");
                                    ce = tupla_infix.0(self,ce).unwrap();

                                },
                                _ => { 
                                    // println!("sale del bucle por infix retornando None"); 
                                    return Some(ce); 
                                }
                            };

                            // println!("sale del bucle normal");
                        }

                        // println!("devolvio bien el left-pex");
                        Some(ce)
                    }, 
                    None => {
                        // println!("devolvio None el left-pex");  
                        None
                    }
                }      
            },

            _ => { 
                // println!("dio None en el prefix"); 
                self.no_prefix_parse_fn_error(self.cur_token.clone()); 
                None
            }
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

        // println!("the stmt exprecion is {:?}", &stmt.expression);
        if self.peek_token_is(Token::SEMICOLON){
            self.next_token();
        }
        Some(stmt)
    }

    pub fn parse_return_statement(&mut self) -> Option<ast::ReturnStatement>{
        let token_var = self.cur_token.clone();
        self.next_token();
        let return_var = self.parse_expression(LOWEST);

        if self.peek_token_is(Token::SEMICOLON){
            self.next_token();
        }

        return Some(
            ast::ReturnStatement{
                token:token_var,
                return_value:return_var
            }
        )
        // let stmt = ast::ReturnStatement{token:self.cur_token.clone() , return_value:None};
        // self.next_token();

        // //TODO: we are skipping the exprescions until we 
        // // Enconunter a semicolon
        // while !self.cur_token_is(Token::SEMICOLON){
        //     self.next_token();
        // }

        // return Some(stmt);
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

        stmt.name =  ast::Identifier{token : self.cur_token.clone(), value: self.cur_token.literal.to_string()};

        if !self.expect_peek(Token::ASSING){
            return None;
        }

        self.next_token();

        stmt.value = self.parse_expression(LOWEST);

        if self.peek_token_is(Token::SEMICOLON){
            self.next_token();
        }

        return Some(stmt);

        // let mut stmt = ast::let_statement{
        //     value : None, 
        //     token : self.cur_token.clone(), 
        //     name: ast::Identifier{
        //         token : TokenType{type_token:Token::EOF,literal:"".to_string()},
        //         value : "".to_string() 
        //     }, 
        // };

        // if !self.expect_peek(Token::IDENT){
        //     return None;
        // }

        // stmt.name = ast::Identifier{token : self.cur_token.clone(), value: self.cur_token.literal.to_string()};

        // if !self.expect_peek(Token::ASSING){
        //     return None
        // } 

        // // TODO: We are Skiping the exprecion until we encounter a semicolon
        // while !self.cur_token_is(Token::SEMICOLON){
        //     self.next_token();
        // }

        // return Some(stmt);

    }

    pub fn cur_token_is (&self, token : Token) -> bool { 
        return self.cur_token.type_token == token;
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
            // println!(" ");
            // println!("entro en el erorr con: {:?},{:?}",self.cur_token.clone(),self.peek_token.clone());
            // println!(" ");
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
                None => {
                    // println!("dio nulo el parse program") 
                    ()
                }

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

    // Prefix operations
    p.register_prefix(TokenType{type_token:Token::IDENT,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_identifier, "parse_identifier".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::INT,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_integer_literal, "parse_integer_literal".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::BANG,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_prefix_expression, "parse_prefix_expression".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::MINUS,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_prefix_expression, "parse_prefix_expression".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::TRUE,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_boolean, "parse_bolean".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::FALSE,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_boolean, "parse_bolean".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::LPAREN,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_grouped_expression,"parse_grouped_expression".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::IF,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_if_expression,"parse_if_expression".to_string()) ));
    p.register_prefix(TokenType{type_token:Token::FUNCTION,literal:"".to_string()}, FUNCIONES::PREFIXPARSEFN2( (Parser::parse_function_literal,"parse_function_literal".to_string()) ));    
    // infix operations
    p.register_Infix(TokenType{type_token:Token::PLUS,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));
    p.register_Infix(TokenType{type_token:Token::MINUS,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));    
    p.register_Infix(TokenType{type_token:Token::SLASH,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));
    p.register_Infix(TokenType{type_token:Token::ASTERISK,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));
    p.register_Infix(TokenType{type_token:Token::EQUAL,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));
    p.register_Infix(TokenType{type_token:Token::NOTEQUAL,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));
    p.register_Infix(TokenType{type_token:Token::LESSTHAN,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));
    p.register_Infix(TokenType{type_token:Token::BIGERTHAN,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_infix_expression, "parse_infix_expression".to_string()) ));
    p.register_Infix(TokenType{type_token:Token::LPAREN,literal:"".to_string()}, FUNCIONES::INFIXPARSEFN( (Parser::parse_call_expression, "parse_call_expression".to_string()) ));

    p.next_token();
    p.next_token();

    return p;

}


