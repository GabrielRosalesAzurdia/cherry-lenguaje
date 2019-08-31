pub mod types;
use types::Token;
// use std::collections::HashMap;

struct Character{
    expected_type : Token,
    expected_literal : String,
}

fn test_next_token(){
    let input = "=+(){},;".to_string();
    // let mut test : HashMap<String, Token > = HashMap::new();
    // test.insert( String::from("="), Token::ASSING );
    // test.insert( String::from("+"), Token::PLUS );
    // test.insert( String::from("("), Token::LPAREN );
    // test.insert( String::from(")"), Token::RPAREN );
    // test.insert( String::from("{"), Token::LBRACE );
    // test.insert( String::from("}"), Token::RBRACE );
    // test.insert( String::from(","), Token::COMMA );
    // test.insert( String::from(";"), Token::SEMICOLON );
    // test.insert( String::from(""), Token::EOF);

    let test = vec![
        Character{ expected_type:Token::ASSING, expected_literal: "=".to_string() },
        Character{ expected_type:Token::PLUS, expected_literal: "+".to_string() },
        Character{ expected_type:Token::LPAREN, expected_literal: "(".to_string() },
        Character{ expected_type:Token::RPAREN, expected_literal: ")".to_string() },
        Character{ expected_type:Token::LBRACE, expected_literal: "{".to_string() },
        Character{ expected_type:Token::RBRACE, expected_literal: "}".to_string() },
        Character{ expected_type:Token::COMMA, expected_literal: ",".to_string() },
        Character{ expected_type:Token::SEMICOLON, expected_literal: ";".to_string() },
        Character{ expected_type:Token::EOF, expected_literal: "".to_string() },
    ];

    let mut l = types::new(input);

    for i in test{
        let tok = l.next_token();
        let x = i.expected_type;

        match tok.type_token {
            x => panic!("joder"),
            _ => println!("bien")
        };

        // if x != i.expected_type{
        //     println!("tomaos mal el type");
        // }

        if tok.literal != i.expected_literal {
            println!("toamos mal el literal")
        }

    }



}