pub mod types;
use types::Token;

#[derive(Debug)]
struct Character{
    expected_type : Token,
    expected_literal : String,
}

#[test]
pub fn test_next_token(){
    let input = "let five = 5;
    let ten = 10;
    let add = fn (x,y){x + y;};
    let result = add(five,ten);
    !-/*5;
    5 < 10 > 5; 

    if (5 < 10){
        return true;
    }
    else {
        return false;
    }

    10 == 10;
    10 != 9;

    ".to_string();

    let test = vec![
        Character{ expected_type:Token::LET, expected_literal: "let".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "five".to_string() },
        Character{ expected_type:Token::ASSING, expected_literal: "=".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "5".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::LET, expected_literal: "let".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "ten".to_string() },
        Character{ expected_type:Token::ASSING, expected_literal: "=".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "10".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::LET, expected_literal: "let".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "add".to_string() },
        Character{ expected_type:Token::ASSING, expected_literal: "=".to_string() },
        Character{ expected_type:Token::FUNCTION, expected_literal: "fn".to_string() },
        Character{ expected_type:Token::LPAREN, expected_literal: "(".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "x".to_string() },
        Character{ expected_type:Token::COMMA, expected_literal: ",".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "y".to_string() },
        Character{ expected_type:Token::RPAREN, expected_literal: ")".to_string() },
        Character{ expected_type:Token::LBRACE, expected_literal: "{".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "x".to_string() },
        Character{ expected_type:Token::PLUS, expected_literal: "+".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "y".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::RBRACE, expected_literal: "}".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::LET, expected_literal: "let".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "result".to_string() },
        Character{ expected_type:Token::ASSING, expected_literal: "=".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "add".to_string() },
        Character{ expected_type:Token::LPAREN, expected_literal: "(".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "five".to_string() },
        Character{ expected_type:Token::COMMA, expected_literal: ",".to_string() },
        Character{ expected_type:Token::IDENT, expected_literal: "ten".to_string() },
        Character{ expected_type:Token::RPAREN, expected_literal: ")".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::BANG, expected_literal: "!".to_string() },
        Character{ expected_type:Token::MINUS, expected_literal: "-".to_string() },
        Character{ expected_type:Token::SLASH, expected_literal: "/".to_string() },
        Character{ expected_type:Token::ASTERISK, expected_literal: "*".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "5".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "5".to_string() },
        Character{ expected_type:Token::LESSTHAN, expected_literal: "<".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "10".to_string() },
        Character{ expected_type:Token::BIGERTHAN, expected_literal: ">".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "5".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::IF, expected_literal: "if".to_string() },
        Character{ expected_type:Token::LPAREN, expected_literal: "(".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "5".to_string() },
        Character{ expected_type:Token::LESSTHAN, expected_literal: "<".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "10".to_string() },
        Character{ expected_type:Token::RPAREN, expected_literal: ")".to_string() },
        Character{ expected_type:Token::LBRACE, expected_literal: "{".to_string() },
        Character{ expected_type:Token::RETURN, expected_literal: "return".to_string() },
        Character{ expected_type:Token::TRUE, expected_literal: "true".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::RBRACE, expected_literal: "}".to_string() },
        Character{ expected_type:Token::ELSE, expected_literal: "else".to_string() },
        Character{ expected_type:Token::LBRACE, expected_literal: "{".to_string() },
        Character{ expected_type:Token::RETURN, expected_literal: "return".to_string() },
        Character{ expected_type:Token::FALSE, expected_literal: "false".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::RBRACE, expected_literal: "}".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "10".to_string() },
        Character{ expected_type:Token::EQUAL, expected_literal: "==".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "10".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "10".to_string() },
        Character{ expected_type:Token::NOTEQUAL, expected_literal: "!=".to_string() },
        Character{ expected_type:Token::INT, expected_literal: "9".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::EOF, expected_literal: "".to_string() },
    ];

    let mut l = types::new(input);

    for i in test{
        
        let tok = l.next_token();
        let _x = i.expected_type;

        // match tok.type_token {
        //     _x => println!("joder"),
        //     _ => println!("bien")
        // };
        println!("{:?}", &_x);

        if _x != tok.type_token{
            panic!("bien type");
        }

        if tok.literal != i.expected_literal {
            panic!("bien literal")
        }

    }



}