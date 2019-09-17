use std::io::{self, BufRead, Write};

pub mod lexer;
pub mod ab_st_tr;
pub mod parser;
use ab_st_tr::ast::Node;

fn main() {
    println!(" ");
    println!("hello, welcome to the cherry programing lenguaje");
    println!("feel free to type comands");
    println!(" ");
    let stdin = io::stdin();

    loop {
        // Stdout needs to be flushed, due to missing newline
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let lexer = lexer::types::new(line);
        let mut p = parser::parser::new(&lexer);

        let program = p.parse_program();

        if p.errors.len() != 0 {
            print_parser_error(p.errors);
            continue;
        }

        println!("{}", program.a_string());
        println!(" ");

        // loop {
        //     let tok = lexer.next_token();
        //     println!("{:?}", tok);
        //     if tok.type_token == lexer::types::Token::EOF {
        //         break;
        //     }
        // }
    }
}

fn print_parser_error(e : Vec<String>){

    let _c =  "
                d888P
      d8b d8888P:::P
    d:::888b::::::P
   d:::dP8888b:d8P
  d:::dP 88b  Yb   .d8888b.
 d::::P  88Yb  Yb .P::::::Y8b
 8:::8   88`Yb  YbP::::::::::b
 8:::P   88 `8   8!:::::::::::b
 8:dP    88  Yb d!!!::::::::::8
 8P    ..88   Yb8!!!::::::::::P
  .d8:::::Yb  d888VKb:!:!::!:8
 d::::::::::dP:::::::::b!!!!8
8!!::::::::P::::::::::::b!8P
8:!!::::::d::::::::::::::b
8:!:::::::8!:::::::::::::8
8:!!!:::::8!:::::::::::::8
Yb:!!:::::8!!::::::::::::8
 8b:!!!:!!8!!!:!:::::!!:dP
   8b:!!!:Yb!!!!:::::!d88
      000  Y88!!!!!!!d8P
              00000000           
";

let c_2 = "                                                                          
                                                      ██                                
                                          ██    ██████                                  
                                          ██  ██▒▒▒▒██                                  
                                        ██████▒▒▒▒▒▒██                                  
                                        ██  ██▒▒▒▒██                                    
                                      ██    ██████                                      
                                    ██    ██                                            
                                    ██    ██                                            
                                  ██        ██                                          
                                ██            ████████                                  
                            ████████        ██▒▒▒▒▒▒▒▒██                                
                          ██▒▒▒▒▒▒▒▒██    ██▒▒▒▒▒▒▒▒▒▒▒▒██                              
                        ██▒▒▒▒▒▒  ▒▒▒▒██  ██▒▒▒▒▒▒▒▒▒▒▒▒██                              
                        ██▒▒▒▒▒▒▒▒  ▒▒██  ██▒▒  ▒▒▒▒▒▒▒▒██                              
                        ██▒▒▒▒▒▒▒▒  ▒▒██  ██▒▒▒▒    ▒▒▒▒██                              
                        ██▒▒▒▒▒▒▒▒▒▒▒▒██    ██▒▒▒▒▒▒▒▒██                                
                          ██▒▒▒▒▒▒▒▒██        ████████                                  
                            ████████                                                                                                                          
";

    println!("{}", c_2);
    println!("Woop we found some errors in the cherry tree.");
    println!("parser errors:");
    for i in e {
        println!("  error: {}", i);
    }
}