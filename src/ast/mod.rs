pub mod ast;
use crate::lexer::types;
use crate::ast::ast::Node;


#[test]
pub fn test_string(){
    let program = ast::Program{
        Statements : vec![
            Box::new(
                ast::let_statement{
                    token : types::TokenType{ type_token:types::Token::LET, literal:"let".to_string()},
                    name : ast::Identifier{ 
                        token: types::TokenType{ type_token:types::Token::IDENT, literal:"myVar".to_string()}, 
                        value:"myVar".to_string()
                    },
                    value : Some(Box::new(ast::Identifier{ 
                        token: types::TokenType{ type_token:types::Token::IDENT, literal:"anotherVar".to_string()}, 
                        value:"anotherVar".to_string()
                    }))
                }
            )
        ]
    };

    if program.a_string() != "let myVar = anotherVar;" {
        println!("{}", program.a_string() );
        panic!("ijole");
    }
}