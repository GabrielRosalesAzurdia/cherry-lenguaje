// use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Token {
    Illegal,
    EOF,    

    // Identifiers + literals 
    IDENT,
    INT,

    // operations 
    ASSING,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    BIGERTHAN,
    LESSTHAN,

    EQUAL,
    NOTEQUAL,

    // Delimiters 
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN 
}


#[derive(Debug)]
pub struct TokenType{
    pub type_token : Token,
    pub literal : String,
}

#[derive(Debug)]
pub struct Lexer {
    input : String,
    position : u32,  // current position in input (points to current char)
    read_position : u32,  // current reading position in input (after current char)
    ch : u8 ,  // current char under examination
}

impl Lexer{
// Read the actual character and refresh
    pub fn read_char(&mut self){

        let a = self.read_position;

        if a >= self.input.len() as u32 {
            self.ch = 0;
        }
        else{
            let x : char = self.input.chars().nth( self.read_position as usize ).unwrap();
            self.ch = x as u8;
        }

        self.position = self.read_position;
        self.read_position += 1;

    }

    pub fn next_token(&mut self) -> TokenType {
        let mut tok : TokenType = TokenType{ type_token:Token::EOF, literal:String::from("") };

        self.skip_whitespace();

        if self.ch != 0 {

            let array_of_u8 = [self.ch];
            let string_complete = String::from_utf8_lossy(&array_of_u8);

            if string_complete == String::from("=") {
                if self.peek_char() == '='{
                    self.read_char();
                    tok = TokenType{ type_token:Token::EQUAL, literal:String::from("==") }
                }else{
                    tok = TokenType{ type_token:Token::ASSING, literal:String::from("=") };
                }
            }
            else if string_complete == String::from("+"){
                tok = TokenType{ type_token:Token::PLUS, literal:String::from("+") };
            }
            else if string_complete == String::from("-") {
                tok = TokenType{ type_token:Token::MINUS, literal:String::from("-") };
            }
            else if string_complete == String::from("!") {
                if self.peek_char() == '='{
                    self.read_char();
                    tok = TokenType{ type_token:Token::NOTEQUAL, literal:String::from("!=") }
                }else{
                tok = TokenType{ type_token:Token::BANG, literal:String::from("!") };
                }
            }
            else if string_complete == String::from("/") {
                tok = TokenType{ type_token:Token::SLASH, literal:String::from("/") };
            }
            else if string_complete == String::from("*") {
                tok = TokenType{ type_token:Token::ASTERISK, literal:String::from("*") };
            }
            else if string_complete == String::from(">") {
                tok = TokenType{ type_token:Token::BIGERTHAN, literal:String::from(">") };
            }
            else if string_complete == String::from("<") {
                tok = TokenType{ type_token:Token::LESSTHAN, literal:String::from("<") };
            }
            else if string_complete == String::from(";"){
                tok = TokenType{ type_token:Token::SEMICOLON, literal:String::from(";") };
            }
            else if string_complete == String::from(","){
                tok = TokenType{ type_token:Token::COMMA, literal:String::from(",") };
            }
            else if string_complete == String::from("{"){
                tok = TokenType{ type_token:Token::LBRACE, literal:String::from("{") };
            }
            else if string_complete == String::from("}"){
                tok = TokenType{ type_token:Token::RBRACE, literal:String::from("}") };
            }
            else if string_complete == String::from("("){
                tok = TokenType{ type_token:Token::LPAREN, literal:String::from("(") };
            }
            else if string_complete == String::from(")"){
                tok = TokenType{ type_token:Token::RPAREN, literal:String::from(")") };
            }
            else{
                if self.is_letter(&self.ch){
                    tok.literal = self.read_identifier();
                    tok.type_token = self.lookup_ident(&tok.literal);
                    return tok;
                }else if  self.is_digit(&self.ch){
                    tok.literal = self.read_number();
                    tok.type_token = Token::INT;
                    return tok;
                }
                else{
                    tok = TokenType{ type_token:Token::Illegal, literal:string_complete.to_string() };
                }
            }

        }else{
            tok = TokenType{ type_token:Token::EOF, literal:String::from("") };
        }

        self.read_char();

        return tok;

    }

    pub fn read_identifier(&mut self) -> String {
        let p = self.position as usize;
        
        while self.is_letter(&self.ch) {
            self.read_char();
        }

        let resultado = &self.input[p..self.position as usize];

        resultado.to_string()
        
    }

    pub fn is_letter(&self,c : &u8 ) -> bool {
        let p1 = 'a' as u8;
        let p2 = 'z' as u8;
        let p3 = 'A' as u8;
        let p4 = 'Z' as u8;
        let p5 = '_' as u8;
        return &p1 <= c && c <= &p2 || &p3 <= c && c <= &p4 || c == &p5;   
    }

    pub fn lookup_ident(&self, literal : &String) -> Token {
        if literal == "fn"{
            return Token::FUNCTION;
        }
        else if literal == "let"{
            return Token::LET;
        }
        else if literal == "true"{
            return Token::TRUE;
        }
        else if literal == "false"{
            return Token::FALSE;
        }
        else if literal == "if"{
            return Token::IF;
        }
        else if literal == "else"{
            return Token::ELSE;
        }
        else if literal == "return"{
            return Token::RETURN;
        }

        return Token::IDENT;
    }

    pub fn skip_whitespace(&mut self){
        while self.ch == ' ' as u8 || self.ch == '\t' as u8 || self.ch == '\n' as u8 || self.ch == '\r' as u8 {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> String {
        let p = self.position as usize;
        
        while self.is_digit(&self.ch) {
            self.read_char();
        }

        let resultado = &self.input[p..self.position as usize];

        resultado.to_string()
    }

    pub fn is_digit(&self, c : &u8) -> bool{
        let num1 = '0' as u8;
        let num2 = '9' as u8 ;
        return &num1 <= c && c <= &num2;
    }

    pub fn peek_char(&self) -> char {
        if self.read_position as usize >= self.input.len(){
            return ' ';
        }else{
            return self.input.chars().nth(self.read_position as usize).unwrap();
        }
    }

}

// New lexer
pub fn new(input:String) -> Lexer{
    let mut l = Lexer{ input:input, position:0, read_position:0, ch:0 };
    l.read_char();
    l
}


