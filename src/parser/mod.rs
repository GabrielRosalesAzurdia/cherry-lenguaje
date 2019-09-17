pub mod parser;
// use super::ast;
use crate::lexer::types;
use crate::ab_st_tr::ast;    
use crate::ab_st_tr::ast::Statement;
use crate::ab_st_tr::ast::Expression;
use crate::ab_st_tr::ast::ExpressionStatement;
use crate::ab_st_tr::ast::Node;

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

#[warn(dead_code)]
struct LetTest <T>{
    input : String,
    expected_ident : String,
    expected_expression : T,
}

#[warn(dead_code)]
struct ReturnTest <T>{
    input : String,
    expected_value : T,
}

#[warn(dead_code)]
struct PrefixTest2 <T> {
    input : String,
    operator : String,
    value : T,
}

#[warn(dead_code)]
struct InfixTest{
    input : String,
    left_value : i32,
    operator : String,
    rigth_value : i32,
}

#[warn(dead_code)]
struct InfixTest2 <T>{
    input : String,
    left_value : T,
    operator : String,
    rigth_value : T,
}

#[warn(dead_code)]
struct OperationPresedence{
    input: String,
    expected : String,
}

#[warn(dead_code)]
struct BooleanTest {
    input : String,
    expected : bool
}

#[warn(dead_code)]
pub struct CDT<T> {
    pub value : T,
    pub valor_des : String,
}

#[warn(dead_code)]
pub struct FunctionParameterParsigTest{
    input : String ,
    expect_params : Vec<String>
}

#[warn(dead_code)]
pub struct TestCallExpressionParameterParsing{
    input : String,
    expected_ident : String,
    expected_args : Vec<String>,
}

#[test]
pub fn test_call_expression_parameter_parsing(){
    let test = vec![
        TestCallExpressionParameterParsing{
            input:"add();".to_string(), 
            expected_ident:"add".to_string(), 
            expected_args:vec![]
        },
        TestCallExpressionParameterParsing{
            input:"add(1);".to_string(), 
            expected_ident:"add".to_string(), 
            expected_args:vec!["1".to_string()]
        },
        TestCallExpressionParameterParsing{
            input:"add(1,2 * 3, 4 + 5);".to_string(), 
            expected_ident:"add".to_string(), 
            expected_args:vec!["1".to_string(), "(2 * 3)".to_string(), "(4 + 5)".to_string() ]}, 
    ];

    for i in test.iter(){
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);

        let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(b) => b,
            None => panic!("program.Statements[0] no es un Expresion statement"),
        };   

        let exp =  match stmt.expression.as_any().downcast_ref::<ast::CallExpression>() {
            Some(b) => b,
            None => panic!("stmt.expression no es un CallExpression"),
        };   

        if !test_identifier(&exp.function, i.expected_ident.to_string()){
            return;
        } 

        if exp.arguments.len() != i.expected_args.len() {
            panic!("numero equivocado de argumentos, esperados: {}, got: {}",i.expected_args.len(),  exp.arguments.len());
        }

        for (j,e) in i.expected_args.iter().enumerate() {
            if exp.arguments[j].a_string() != e.to_string(){
                panic!("mal argument, esperado: {}, got: {}", e, exp.arguments[j].a_string());
            }
        }

    }
}

#[test]
pub fn test_call_expression_parsing(){
    let input = "add(1, 2 * 3, 4 + 5);".to_string();
    let l = types::new(input.to_string());
    let mut p = parser::new(&l);
    let program = p.parse_program();
    check_parser_errors(&p);   

    let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("program.Statements[0] no es un Expresion statement"),
    };   

    let exp =  match stmt.expression.as_any().downcast_ref::<ast::CallExpression>() {
        Some(b) => b,
        None => panic!("stmt.expression no es un CallExpression"),
    };   

    if !test_identifier(&exp.function, "add".to_string()){
        return;
    }

    if exp.arguments.len() != 3{
        panic!("mal cantidad de argumentos,got: {}",exp.arguments.len());
    }

    let x = CDT{
        valor_des: "valor entero".to_string(),
        value : 1
    };

    x.test_literal_expression(&exp.arguments[0]);

    let l1 = CDT{
        valor_des: "operacion 1".to_string(),
        value : 2
    };

    let r1 = CDT{
        valor_des: "operacion 1".to_string(),
        value : 3
    };

    let l2 = CDT{
        valor_des: "operacion 2".to_string(),
        value : 4
    };

    let r2 = CDT{
        valor_des: "operacion 2".to_string(),
        value : 5
    };

    test_infix_expression(&exp.arguments[1], l1, "*".to_string(), r1);
    test_infix_expression(&exp.arguments[2], l2, "+".to_string(), r2);
}

#[test]
pub fn test_function_parameter_parsing(){
    let test = vec![
        FunctionParameterParsigTest{ input:"fn() {};".to_string(), expect_params:vec![] },
        FunctionParameterParsigTest{ input:"fn(x) {};".to_string(), expect_params:vec!["x".to_string()] },
        FunctionParameterParsigTest{ input:"fn(x, y, z) {};".to_string(), expect_params:vec!["x".to_string(), "y".to_string(), "z".to_string()] }
    ];

    for i in test.iter(){
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);


        let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(b) => b,
            None => panic!("program.Statements[0] no es un Expresion statement"),
        };   

        let funct =  match stmt.expression.as_any().downcast_ref::<ast::FunctionLiteral>() {
            Some(b) => b,
            None => panic!("stmt.expression no es un functionliteral"),
        };   

        if funct.parameters.len() != i.expect_params.len(){
            panic!("el fucnt.parameters es: {}, y el i.expect_params es: {}", funct.parameters.len(), i.expect_params.len());
        }

        for (j,e) in i.expect_params.iter().enumerate() {
            let x = CDT{
                valor_des : "valor dentro del vector de nombre de variables".to_string(),
                value : e.to_string()
            };

            let y : Box<(dyn ast::Expression + 'static)> = Box::from(  ast::Identifier{ token: funct.parameters[j].token.clone(), value : funct.parameters[j].value.to_string() } );

            x.test_literal_expression( &y );
        }

    }
}

#[test]
pub fn test_function_literal_parsing(){
    let input = "fn(x, y) { x + y; }".to_string();
    let l = types::new(input.to_string());
    let mut p = parser::new(&l);
    let program = p.parse_program();
    check_parser_errors(&p);

    if program.Statements.len() != 1 {
        panic!("progra.Statements no tiene un statemet, got: {}", program.Statements.len());
    }

    let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("program.Statements[0] no es un Expresion statement"),
    };

    let funct =  match stmt.expression.as_any().downcast_ref::<ast::FunctionLiteral>() {
        Some(b) => b,
        None => panic!("stmt.expression no es un FunctionLiteral"),
    };
                    
    if funct.parameters.len() != 2 {
        panic!("funct.parameters no tiene 2 parametros, got: {}", funct.parameters.len());
    }

    let px = CDT{
        valor_des : "parametro x".to_string(),
        value : "x".to_string()
    };

    let py = CDT{
        valor_des : "parametro y".to_string(),
        value : "y".to_string()
    };

    let x : Box<(dyn ast::Expression + 'static)> = Box::from(  ast::Identifier{ token: funct.parameters[0].token.clone(), value : funct.parameters[0].value.to_string() } );
    let y : Box<(dyn ast::Expression + 'static)> = Box::from(  ast::Identifier{ token: funct.parameters[1].token.clone(), value : funct.parameters[1].value.to_string() } );

    px.test_literal_expression( &x );
    py.test_literal_expression( &y );

    if funct.body.statements.len() != 1 {
        panic!("funct.body.statements no tiene 1 statement, got: {}",funct.body.statements.len());
    }

    let bodystmt = match funct.body.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("funct.body.statements[0] no es un expressionstatement"),
    };

    let l  = CDT{
        valor_des: "es letra x".to_string(),
        value : "x".to_string(),
    };

    let r  = CDT{
        valor_des: "es letra y".to_string(),
        value : "y".to_string(),
    };

    test_infix_expression( &bodystmt.expression , l , "+".to_string() , r);

}

#[test]
pub fn test_if_else_expression(){
    let input = "if (x < y) { x } else { y }".to_string();
    let l = types::new(input.to_string());
    let mut p = parser::new(&l);
    let program = p.parse_program();
    check_parser_errors(&p);

    if program.Statements.len() != 1 {
        panic!("program.statements no tiene 1 statement tiene , got: {}", program.Statements.len());
    }

    let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("program.Statements[0] no es un Expresion statement"),
    };

    let exp =  match stmt.expression.as_any().downcast_ref::<ast::IfExpression>() {
        Some(b) => b,
        None => panic!("stmt.expression no es un IfExpression"),
    };

    let l = CDT{
        valor_des: "es una x".to_string(),
        value : "x".to_string()
    };

    let r = CDT{
        valor_des: "es una y".to_string(),
        value: "y".to_string()
    };

    if !test_infix_expression(&exp.condition,  l, "<".to_string(), r){
        return;
    }

    let x = match &exp.consequence{
        Some(c) => c ,
        None => panic!("no hay consequence")
    };

    if x.statements.len() != 1 {
        panic!("consequence no tiene 1 statement tiene, got: {}");
    }     

    let conse =  match x.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("x.statements[0] no es un ExpressionStatement"),
    };

    if !test_identifier(&conse.expression, "x".to_string()){
        return;
    }

    let y = match &exp.alternative{
        Some(c) => c ,
        None => panic!("no hay alternative")
    };

    if y.statements.len() != 1 {
        panic!("alternative no tiene 1 statement, got: {}", y.statements.len());
    }

    let alter =  match y.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("y.statemenets[0] no es un ExpressionStatement"),
    };

    if !test_identifier(&alter.expression, "y".to_string()){
        return;
    }

}

#[test]
pub fn test_if_expression(){
    let input = "if (x < y) { x } ".to_string();
    let l = types::new(input.to_string());
    let mut p = parser::new(&l);
    let program = p.parse_program();
    check_parser_errors(&p);

    if program.Statements.len() != 1 {
        panic!("el cuerpo del programa no contiene 1 sino, got: {}", program.Statements.len());
    }

    let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("program.Statements[0] no es un Expresion statement"),
    };

    let exp =  match stmt.expression.as_any().downcast_ref::<ast::IfExpression>() {
        Some(b) => b,
        None => panic!("stmt.expression no es un IfExpression"),
    };

    let l = CDT{
        valor_des: "es una x".to_string(),
        value : "x".to_string()
    };

    let r = CDT{
        valor_des: "es una y".to_string(),
        value: "y".to_string()
    };

    if !test_infix_expression(&exp.condition,  l, "<".to_string(), r){
        return;
    }

    let x = match &exp.consequence{
        Some(c) => c ,
        None => panic!("no hay consequence")
    };

    if x.statements.len() != 1 {
        panic!("consequence no tiene 1 statement tiene, got: {}");
    } 

    let conse =  match x.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
        Some(b) => b,
        None => panic!("x.statements[0] no es un ExpressionStatement"),
    };

    if !test_identifier(&conse.expression, "x".to_string()){
        return;
    }

    match exp.alternative {
        Some(_) => panic!("no deveria haver un alternative"),
        None => () 
    };

}

#[test]
pub fn test_boolean_expression(){
    let test = [
        BooleanTest{ input:"true".to_string(), expected: true },
        BooleanTest{ input:"false".to_string(), expected: false } 
    ];

    for i in test.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.Statements.len() != 1 {
            panic!("hay mas de un statement, got: {}", program.Statements.len());
        }

        let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(b) => b,
            None => panic!("Statements[0] no es un expression statement"),
        };

        let boolen =  match stmt.expression.as_any().downcast_ref::<ast::Boolean>() {
            Some(b) => b,
            None => panic!("stmt.expression no es un Bolean"),
        };

        println!("el bolean es {:?}", boolen);

        if boolen.value != i.expected{
            panic!("boolean value no es {}, got {}", i.expected, boolen.value);
        }

    }
}

// With these in place it’s possible to write test code like this:
// testInfixExpression(t, stmt.Expression, 5, "+", 10)
// testInfixExpression(t, stmt.Expression, "alice", "*", "bob")

pub fn test_infix_expression (exp:&Box<dyn ast::Expression>, left: impl TLE, operator:String, rigth: impl TLE ) -> bool {
    let infi =  match exp.as_any().downcast_ref::<ast::InfixExpression>() {
        Some(b) => b,
        None => panic!("exp no es un infix expression"),
    };

    if !left.test_literal_expression(&infi.left ){
        return false
    }

    if infi.operator != operator {
        return false
    }

    if !rigth.test_literal_expression(&infi.rigth ){
        return false
    }

    true

}

//---------------------------------------------------------------------------------------------------

// impl<T:Display> CDT<T> {

// }

pub trait TLE{
    fn test_literal_expression(&self,exp:&Box<dyn ast::Expression>) -> bool;
}

impl TLE for CDT<i32>{
    fn test_literal_expression (&self, exp : &Box<dyn ast::Expression>) -> bool {
        test_integer_literal(exp, self.value )
    }
}

impl TLE for CDT<String>{
    fn test_literal_expression (&self, exp : &Box<dyn ast::Expression>) -> bool {
        test_identifier(exp, self.value.to_string())
    }
}

impl TLE for CDT<bool>{
    fn test_literal_expression (&self, exp : &Box<dyn ast::Expression>) -> bool {
        test_bool_literal(exp, self.value )
    }
}

// pub fn test_literal_expression  (exp : &Box<dyn ast::Expression>, expected : CDT<String> ) -> bool {

//     if expected.valor_des == "i32".to_string(){
//         return test_integer_literal(exp, expected.value );
//     }
//     else if expected.valor_des == "String".to_string(){
//         return test_identifier(exp, expected.value ); 
//     }
//     else if expected.valor_des == "bool".to_string(){
//         return test_bool_literal(exp, expected.value );
//     }
//     else{
//         panic!("tipo no tomado en cuenta, got: {}",expected.valor_des);
//     }

// }

//---------------------------------------------------------------------------------------------------


pub fn test_bool_literal(exp : &Box<dyn ast::Expression>, value: bool) -> bool{

    let bo = match exp.as_any().downcast_ref::<ast::Boolean>() {
        Some(b) => b,
        None => panic!("exp no es un Boolean"),
    };

    if bo.value != value{
        panic!("bo.value no es {}, got: {}",bo.value, value);
    }

    if bo.token_literal() != format!("{}",value){
        panic!("bo.token_literal() no es {}, got: {}", value, bo.token_literal());
    }

    true
}

pub fn test_identifier(exp : &Box<dyn ast::Expression>, value: String) -> bool{
    let ident =  match exp.as_any().downcast_ref::<ast::Identifier>() {
        Some(b) => b,
        None => panic!("exp no es un expression statement"),
    };

    if ident.value != value {
        panic!("ident.value no es {}, got: {}",value,ident.value);
    }

    if ident.token_literal() != value {
        panic!("ident.token_literal() no es {}, got: {}",value, ident.token_literal());
    }

    true
}

#[test]
pub fn test_operation_presedence_parsing (){
    let test = [                
        OperationPresedence{ input:"-a * b".to_string(), expected:"((-a) * b)".to_string() },
        OperationPresedence{ input:"!-a".to_string(), expected:"(!(-a))".to_string() },
        OperationPresedence{ input:"a + b + c".to_string(), expected:"((a + b) + c)".to_string() },
        OperationPresedence{ input:"a + b - c".to_string(), expected:"((a + b) - c)".to_string() },
        OperationPresedence{ input:"a * b * c".to_string(), expected:"((a * b) * c)".to_string() },
        OperationPresedence{ input:"a * b / c".to_string(), expected:"((a * b) / c)".to_string() },
        OperationPresedence{ input:"a + b / c".to_string(), expected:"(a + (b / c))".to_string() },
        OperationPresedence{ input:"a + b * c + d / e - f".to_string(), expected:"(((a + (b * c)) + (d / e)) - f)".to_string() },
        OperationPresedence{ input:"3 + 4; -5 * 5".to_string(), expected:"(3 + 4)((-5) * 5)".to_string() },
        OperationPresedence{ input:"5 > 4 == 3 < 4".to_string(), expected:"((5 > 4) == (3 < 4))".to_string() },
        OperationPresedence{ input:"5 < 4 != 3 > 4".to_string(), expected:"((5 < 4) != (3 > 4))".to_string() },
        OperationPresedence{ input:"3 + 4 * 5 == 3 * 1 + 4 * 5".to_string(), expected:"((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".to_string() },
        OperationPresedence{ input:"3 + 4 * 5 == 3 * 1 + 4 * 5".to_string(), expected:"((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".to_string() },
        OperationPresedence{ input:"true".to_string(), expected:"true".to_string() },
        OperationPresedence{ input:"false".to_string(), expected:"false".to_string() },        
        OperationPresedence{ input:"3 > 5 == false".to_string(), expected:"((3 > 5) == false)".to_string() },
        OperationPresedence{ input:"3 < 5 == true".to_string(), expected:"((3 < 5) == true)".to_string() },
        OperationPresedence{ input:"1 + (2 + 3) + 4".to_string(), expected:"((1 + (2 + 3)) + 4)".to_string() },
        OperationPresedence{ input:"(5 + 5) * 2".to_string(), expected:"((5 + 5) * 2)".to_string() },
        OperationPresedence{ input:"2 / (5 + 5)".to_string(), expected:"(2 / (5 + 5))".to_string() },
        OperationPresedence{ input:"-(5 + 5)".to_string(), expected:"(-(5 + 5))".to_string() },
        OperationPresedence{ input:"!(true == true)".to_string(), expected:"(!(true == true))".to_string() },
        OperationPresedence{ input:"a + add(b * c) + d".to_string(), expected:"((a + add((b * c))) + d)".to_string() },
        OperationPresedence{ input:"add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))".to_string(), expected:"add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))".to_string() },
        OperationPresedence{ input:"add(a + b + c * d / f + g)".to_string(), expected:"add((((a + b) + ((c * d) / f)) + g))".to_string() },
    ];

    for i in test.iter() {

        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);

        let actual = program.a_string();
        if actual != i.expected{
            panic!("esperando: {}, recivido: {}", i.expected, actual);
        }

    }

}

#[test]
pub fn test_parsing_infix_expressions(){
    let infix_test_var = [
        InfixTest{ input: "5 + 5;".to_string(), left_value: 5, operator: "+".to_string(), rigth_value: 5 },
        InfixTest{ input: "5 - 5;".to_string(), left_value: 5, operator: "-".to_string(), rigth_value: 5 },
        InfixTest{ input: "5 * 5;".to_string(), left_value: 5, operator: "*".to_string(), rigth_value: 5 },
        InfixTest{ input: "5 / 5;".to_string(), left_value: 5, operator: "/".to_string(), rigth_value: 5 },
        InfixTest{ input: "5 > 5;".to_string(), left_value: 5, operator: ">".to_string(), rigth_value: 5 },
        InfixTest{ input: "5 < 5;".to_string(), left_value: 5, operator: "<".to_string(), rigth_value: 5 },
        InfixTest{ input: "5 == 5;".to_string(), left_value: 5, operator: "==".to_string(), rigth_value: 5 },
        InfixTest{ input: "5 != 5;".to_string(), left_value: 5, operator: "!=".to_string(), rigth_value: 5 },        
    ];   

    let infix_test_var_c_1 = [
        InfixTest2{ input: "true == true".to_string(), left_value:true, operator:"==".to_string(), rigth_value:true },
        InfixTest2{ input: "true != false".to_string(), left_value:true, operator:"!=".to_string(), rigth_value:false },
        InfixTest2{ input: "false == false".to_string(), left_value:false, operator:"==".to_string(), rigth_value:false },
    ];

    for i in infix_test_var.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.Statements.len() != 1{
            panic!("statements del programa no contienen 1 statement , got: {}", program.Statements.len());
        }

        let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(b) => b,
            None => panic!("Statements[0] no es un expression statement"),
        };

        let desem = &stmt.expression;

        let sta = match  desem.as_any().downcast_ref::<ast::InfixExpression>() {
            Some(b) => b,
            None => panic!("stmt expression no es un Infix Expression")   
        };    

        if !test_integer_literal(&sta.left, i.left_value){
            return;
        }

        if sta.operator != i.operator {
            panic!("exp.operator no es {}, got: {}", i.operator, sta.operator);
        }

        if !test_integer_literal(&sta.rigth, i.rigth_value){
            return;
        }

    };

    for i in infix_test_var_c_1.iter(){

        println!("entro al segundo bucle");

        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.Statements.len() != 1{
            panic!("statements del programa no contienen 1 statement , got: {}", program.Statements.len());
        }

        let stmt =  match program.Statements[0].as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(b) => b,
            None => panic!("Statements[0] no es un expression statement"),
        };

        let desem = &stmt.expression;

        let sta = match  desem.as_any().downcast_ref::<ast::InfixExpression>() {
            Some(b) => b,
            None => panic!("stmt expression no es un Infix Expression")   
        };    

        let l = CDT{ value: i.left_value, valor_des:"bool".to_string() };

        let r = CDT{value : i.rigth_value, valor_des:"bool".to_string()};

        if !l.test_literal_expression(&sta.left){
            return;
        }

        if !r.test_literal_expression(&sta.rigth){
            return;
        }

    };

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

    let prefix_test_var_c_1 = [
        PrefixTest2{ input:"!true".to_string(), operator:"!".to_string(), value:true },
        PrefixTest2{ input:"!false".to_string(), operator:"!".to_string(), value:false }
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

    for i in prefix_test_var_c_1.iter() {

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

        if !test_bool_literal(&sta.rigth,i.value){
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
pub fn test_return_statement(){

    let test_1 = [
        ReturnTest{ input:"return 5;".to_string(), expected_value: 5 }
    ];

    let test_2 = [
        ReturnTest{ input:"return true;".to_string(), expected_value: true }
    ];

    let test_3 = [
        ReturnTest{ input:"return foobar;".to_string(), expected_value: "foobar".to_string() }
    ];

    for i in test_1.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.Statements.len() != 1 { panic!("program statements no tiene un statement,got: {}", program.Statements.len()); }
        let stmt = &program.Statements[0];
        let returnstmt =  match stmt.as_any().downcast_ref::<ast::ReturnStatement>() { Some(b) => b, None => panic!("stmt no es un returnstatement"), };
        if returnstmt.token_literal() != "return".to_string() {
            panic!("returnStmt.TokenLiteral no es 'return,got: {}", returnstmt.token_literal());
        }
        let x = CDT{ valor_des:"valor a evaluar".to_string(), value:i.expected_value };
        x.test_literal_expression(returnstmt.return_value.as_ref().unwrap());
    }

    for i in test_2.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.Statements.len() != 1 { panic!("program statements no tiene un statement,got: {}", program.Statements.len()); }
        let stmt = &program.Statements[0];
        let returnstmt =  match stmt.as_any().downcast_ref::<ast::ReturnStatement>() { Some(b) => b, None => panic!("stmt no es un returnstatement"), };
        if returnstmt.token_literal() != "return".to_string() {
            panic!("returnStmt.TokenLiteral no es 'return,got: {}", returnstmt.token_literal());
        }
        let x = CDT{ valor_des:"valor a evaluar".to_string(), value:i.expected_value };
        x.test_literal_expression(returnstmt.return_value.as_ref().unwrap());
    }

    for i in test_3.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.Statements.len() != 1 { panic!("program statements no tiene un statement,got: {}", program.Statements.len()); }
        let stmt = &program.Statements[0];
        let returnstmt =  match stmt.as_any().downcast_ref::<ast::ReturnStatement>() { Some(b) => b, None => panic!("stmt no es un returnstatement"), };
        if returnstmt.token_literal() != "return".to_string() {
            panic!("returnStmt.TokenLiteral no es 'return,got: {}", returnstmt.token_literal());
        }
        let x = CDT{ valor_des:"valor a evaluar".to_string(), value:i.expected_value.to_string() };
        x.test_literal_expression(returnstmt.return_value.as_ref().unwrap());
    }

    // let input =  "
    // return 2;
    // return 234234;
    // return 10;
    // ".to_string();

    // let l = types::new(input);
    // let mut p = parser::new(&l);    

    // let program = p.parse_program();
    // check_parser_errors(&p);

    // if program.Statements.len() != 3 {
    //     panic!("no hay tres statements, hay: {}", &program.Statements.len());
    // }

    // for (i,e) in program.Statements.iter().enumerate(){

    //     let sta =  match e.as_any().downcast_ref::<ast::ReturnStatement>() {
    //         Some(b) => b,
    //         None => panic!("&a no es B!"),
    //     };

    //     if e.token_literal() != "return"{
    //         panic!("no es un return es un: {}", e.token_literal());
    //     }

    // }

}

#[test]
pub fn test_let_statement(){

    let test_1 = [
        LetTest{ input:"let x = 5".to_string(), expected_ident:"x".to_string(), expected_expression:5 }
    ];

    let test_2 = [
        LetTest{ input:"let y = true".to_string(), expected_ident:"y".to_string(), expected_expression:true }
    ];

    let test_3 = [
        LetTest{ input:"let foobar = y".to_string(), expected_ident:"foobar".to_string(), expected_expression:"y".to_string() }
    ];

    for i in test_1.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.Statements.len() != 1 { panic!("program statements no tiene un statement,got: {}", program.Statements.len()); }
        let stmt = &program.Statements[0];
        if !test_let_statement_internal(stmt, i.expected_ident.to_string()){ return; }
        let val =  match stmt.as_any().downcast_ref::<ast::let_statement>() { Some(b) => b, None => panic!("Statements[0] no es un let statement"), };
        let x = CDT{ valor_des:"valor a evaluar".to_string(), value:i.expected_expression };
        x.test_literal_expression(val.value.as_ref().unwrap());
    }

    for i in test_2.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.Statements.len() != 1 { panic!("program statements no tiene un statement,got: {}", program.Statements.len()); }
        let stmt = &program.Statements[0];
        if !test_let_statement_internal(stmt, i.expected_ident.to_string()){ return; }
        let val =  match stmt.as_any().downcast_ref::<ast::let_statement>() { Some(b) => b, None => panic!("Statements[0] no es un let statement"), };
        let x = CDT{ valor_des:"valor a evaluar".to_string(), value:i.expected_expression };
        x.test_literal_expression(val.value.as_ref().unwrap());
    }

    for i in test_3.iter() {
        let l = types::new(i.input.to_string());
        let mut p = parser::new(&l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.Statements.len() != 1 { panic!("program statements no tiene un statement,got: {}", program.Statements.len()); }
        let stmt = &program.Statements[0];
        if !test_let_statement_internal(stmt, i.expected_ident.to_string()){ return; }
        let val =  match stmt.as_any().downcast_ref::<ast::let_statement>() { Some(b) => b, None => panic!("Statements[0] no es un let statement"), };
        let x = CDT{ valor_des:"valor a evaluar".to_string(), value:i.expected_expression.to_string() };
        x.test_literal_expression(val.value.as_ref().unwrap());
    }

    // let input = "
    // let x = 4;
    // let y = 9;
    // let foo = 838383;   
    // ".to_string();

    // let l = types::new(input);
    // let mut p = parser::new(&l);
    
    // let program = p.parse_program();
    // check_parser_errors(&p);

    // if program.Statements.is_empty() {
    //     panic!("retorno nulo");
    // }
    
    // println!("el statement: {:?}", program.Statements  );

    // if program.Statements.len() != 3 {
    //     panic!("Es mayor que tres statements, dio: {}", program.Statements.len());
    // }


    // let test = vec![ 
    //     EI{expected_identifier:"x".to_string()}, 
    //     EI{expected_identifier:"y".to_string()},
    //     EI{expected_identifier:"foo".to_string()},  
    // ];

    // for (i,e) in test.iter().enumerate(){
    //     let stmt = &program.Statements[i];
    //     if !test_let_statement_internal(&stmt, e.expected_identifier.to_string()){
    //         return;
    //     }
    // }

}

fn test_let_statement_internal(s : &Box<dyn ast::Statement>, name:String ) -> bool {

    if s.token_literal() != "let"{
        panic!("not let");
    }

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