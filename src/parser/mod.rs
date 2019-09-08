use crate::ast::ast;
use crate::lexer::types;
use crate::ast::ast::Statement;
use crate::ast::ast::Expression;
use crate::ast::ast::ExpressionStatement;
use crate::ast::ast::Node;
mod parser;

#[warn(dead_code)]
struct EI {
    expected_identifier : String,
}

#[warn(dead_code)]
struct PrefixTest{
    input : String,
    operator : String,
    integer_value : i32,
}

#[test]
pub fn test_parsing_prefix_expressions(){
    let prefix_test_var = [
        PrefixTest{
            input:"!5;".to_string(),
            operator : "!".to_string(),
            integer_value : 5,
        },
        PrefixTest{
            input:"-15;".to_string(),
            operator : "-".to_string(),
            integer_value : 15,
        }
    ];

    for i in prefix_test_var.iter() {

        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.Statements.len() != 1 {
            panic!("statements del programa no contienen 1 statement, got: {}", program.Statements.len());
        } 

        let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(b) => b,
            None => panic!("Statements[0] no es un expression statement"),
        };

        println!("el stmt es {}", stmt.expression.a_string() );
        let desem = &stmt.expression;

        let sta = match  desem.as_any().downcast_ref::<ast::PrefixExpression>() {
            Some(b) => b,
            None => panic!("stmt expression no es un Prefix Expression")   
        };    

        if sta.operator != i.operator{
            panic!("el operador en i: {}, es diferente de got: {}", i.operator, sta.operator);
        }

        if !test_integer_literal(&sta.rigth,i.integer_value){
            return;
        }

    }

}

pub fn test_integer_literal(il : &Box<dyn ast::Expression>, value : i32) -> bool{
        let integ    =  match il.as_any().downcast_ref::<ast::IntegerLiteral>() {
            Some(b) => b,
            None => panic!("Statements[0] no es un integer literal"),
        };

        if integ.value != value {
            panic!("integ value no es {}, got: {}", value, integ.value);
        }

        if integ.token_literal() != format!("{}",value){
            panic!("integ token literal no es {}, got: {}",value,integ.token_literal());
        }

        true
}

#[test]
pub fn test_integer_literal_expression(){

    let input = "5;".to_string();
    let l = types::new(input);
    let mut p = parser::new(&l);    
    
    let program = p.parse_program();
    check_parser_errors(&p);

    if program.Statements.len() != 1 {
        panic!("statements del programa no contienen 1 statement, got: {}", program.Statements.len());
    }   

    let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("Statements[0] no es un expression statement"),
    };

    let desem = &stmt.expression;

    let sta = match  desem.as_any().downcast_ref::<ast::IntegerLiteral>() {
        Some(b) => b,
        None => panic!("stmt expression no es un Integer Literal")   
    };    

    if sta.value != 5 {
        panic!("literal value no es 5, got: {}", sta.value);
    }

    if sta.token_literal() != "5" {
        panic!("token literal no es 5, got: {}", sta.token_literal());
    }

}

#[test]
pub fn test_identifier_exprecion(){
    let input = "foobar;".to_string();
    let l = types::new(input);
    let mut p = parser::new(&l);    

    let program = p.parse_program();
    check_parser_errors(&p);

    if program.Statements.len() != 1 {
        panic!("statements del programa no contienen 1 statement, got: {}", program.Statements.len());
    }

    let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("Statements[0] no es un expression statement!"),
    };

    let desem = &stmt.expression;

    let sta = match  desem.as_any().downcast_ref::<ast::Identifier>() {
        Some(b) => b,
        None => panic!("stmt expression no es un indentifier")   
    };

    if sta.value != "foobar" {
        panic!("sta.value no es foobar, got: {}", sta.value);
    }

    if sta.token_literal() != "foobar"{
        panic!("sta.token_literal() no es foobar, got: {}", sta.token_literal());
    }

}

#[test]
#[should_panic]
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
            None => panic!("&a no es B!"),
        };

        if e.token_literal() != "return"{
            panic!("no es un return es un: {}", e.token_literal());
        }

    }

}

#[test]
#[should_panic]
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
    
    println!("el statement: {:?}", program.Statements  );

    if program.Statements.len() != 3 {
        panic!("Es mayor que tres statements, dio: {}", program.Statements.len());
    }


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
        None => panic!("&a no es B!"),
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
    panic!("parser tiene {} errores, y son: {:?}",errors.len(), errors);
}