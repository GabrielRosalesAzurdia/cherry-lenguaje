pub mod types;
use types::Token;

struct Character{
    expected_type : Token,
    expected_literal : String,
}

pub fn test_next_token(){
    let input = "=+(){},;".to_string();

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
        let _x = i.expected_type;

        // match tok.type_token {
        //     _x => println!("joder"),
        //     _ => println!("bien")
        // };

        if _x != tok.type_token{
            panic!("bien type");
        }

        if tok.literal != i.expected_literal {
            panic!("bien literal")
        }

    }



}