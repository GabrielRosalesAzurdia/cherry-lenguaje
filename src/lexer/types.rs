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

    // Delimiters 
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET 
}

pub struct TokenType{
    pub type_token : Token,
    pub literal : String,
}

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

        if self.ch != 0 {

            let array_of_u8 = [self.ch];
            let string_complete = String::from_utf8_lossy(&array_of_u8);

            if string_complete == String::from("=") {
                tok = TokenType{ type_token:Token::ASSING, literal:String::from("=") };
            }else if string_complete == String::from(";"){
                tok = TokenType{ type_token:Token::SEMICOLON, literal:String::from(";") };
            }else if string_complete == String::from("{"){
                tok = TokenType{ type_token:Token::LBRACE, literal:String::from("{") };
            }else if string_complete == String::from("}"){
                tok = TokenType{ type_token:Token::RBRACE, literal:String::from("}") };
            }else if string_complete == String::from(","){
                tok = TokenType{ type_token:Token::COMMA, literal:String::from(",") };
            }else if string_complete == String::from("+"){
                tok = TokenType{ type_token:Token::PLUS, literal:String::from("+") };
            }else if string_complete == String::from("("){
                tok = TokenType{ type_token:Token::LPAREN, literal:String::from("(") };
            }else if string_complete == String::from(")"){
                tok = TokenType{ type_token:Token::RPAREN, literal:String::from(")") };
            }else if string_complete == String::from("}"){
                tok = TokenType{ type_token:Token::RBRACE, literal:String::from("}") };
            }

        }else{
            tok = TokenType{ type_token:Token::EOF, literal:String::from("") };
        }

        self.read_char();

        return tok;

    }

}

// New lexer
pub fn new(input:String) -> Lexer{
    let mut l = Lexer{ input:input, position:0, read_position:0, ch:0 };
    l.read_char();
    l
}


