use crate::lexer::types;
use crate::ast::Node;
use crate::ast;
mod parser;

#[warn(dead_code)]
struct EI {
    expected_identifier : String,
}

#[test]
pub fn test_return_statement(){
    let input =  "
    return 2;
    return 234234;
    return 10;
    ".to_string();

    let l = types::new(input);
    let mut p = parser::new(&l);    

    let program = p.parse_program();
    check_parser_errors(&p);

    if program.Statements.len() != 3 {
        panic!("no hay tres statements, hay: {}", &program.Statements.len());
    }

    for (i,e) in program.Statements.iter().enumerate(){

        let sta =  match e.as_any().downcast_ref::<ast::ReturnStatement>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };

        if e.token_literal() != "return"{
            panic!("not is a return is a {}", e.token_literal());
        }

    }

}

#[test]
pub fn test_let_statement(){
    let input = "
    let x = 4;
    let y = 9;
    let foo = 838383;
    ".to_string();

    let l = types::new(input);
    let mut p = parser::new(&l);
    
    let program = p.parse_program();
    check_parser_errors(&p);

    if program.Statements.is_empty() {
        panic!("retorno nulo");
    }

    if program.Statements.len() != 3 {
        panic!("Es mayor que tres statements");
    }

    println!("el statement: {:?}", program.Statements  );

    let test = vec![ 
        EI{expected_identifier:"x".to_string()}, 
        EI{expected_identifier:"y".to_string()},
        EI{expected_identifier:"foo".to_string()},  
    ];

    for (i,e) in test.iter().enumerate(){
        let stmt = &program.Statements[i];
        if !test_let_statement_internal(&stmt, e.expected_identifier.to_string()){
            return;
        }
    }

}

fn test_let_statement_internal(s : &Box<dyn ast::Statement>, name:String ) -> bool {

    if s.token_literal() != "let"{
        panic!("not let");
    }

    // let sta = ast::let_statement{
    //     value : None, 
    //     token : types::TokenType{type_token:types::Token::EOF,literal:"".to_string()}, 
    //     name: ast::Identifier{
    //         token : types::TokenType{type_token:types::Token::EOF,literal:"".to_string()},
    //         value : "".to_string() 
    //     }, 
    // };

    let sta =  match s.as_any().downcast_ref::<ast::let_statement>() {
        Some(b) => b,
        None => panic!("&a isn't a B!"),
    };;


    if sta.name.value != name {
        println!("estos son los nombres:  p: {}  s:{}", &sta.name.value, &name);
        panic!("name mal");
    }

    if sta.name.token_literal() != name {
        panic!("salio mal el nombre 2");
    }
    

    return true;
}

fn check_parser_errors(p: &parser::Parser){
    let errors = p.errors.clone();
    if errors.len() == 0{
        return;
    }
    panic!("parser have {} errors, and are: {:?}",errors.len(), errors);
}