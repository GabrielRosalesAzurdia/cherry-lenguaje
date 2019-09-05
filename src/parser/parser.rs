use crate::lexer::types::TokenType;
use crate::lexer::types::Lexer;
use crate::lexer::types::Token;
use crate::ast;

pub enum ResultadoParseStatement {
    ParseLetStatement(Option<ast::let_statement>),
    ParseResultStatement(Option<ast::ReturnStatement>),
}

pub struct Parser {
    l : Lexer,
    pub errors : Vec<String>,
    cur_token : TokenType,
    peek_token : TokenType,
}

impl Parser{

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
         
        if self.cur_token.type_token == Token::RETURN{
            return match self.parse_return_statement(){
                Some(r) => Some(ResultadoParseStatement::ParseResultStatement(Some(r))),
                None => None,
            };
        }


        return None;    
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
                    };
                },
                None => println!("dio nulo") 

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
        peek_token: TokenType{ type_token:Token::EOF ,literal:"".to_string()}
    };

    p.next_token();
    p.next_token();

    return p;

}