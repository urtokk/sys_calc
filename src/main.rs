use std::io;

use parsemath::parser::{ParseError, Parser};

use crate::parsemath::ast;


mod parsemath;

fn main() {
    println!("Hello. Welcome to Arithmetic expression evaluator.");
    println!("Yeu can calculate value for expressions such as 2*3+(4-5)+2^3/4. ");
    println!("Allowed numbers: positive, negative and decimals.");
    println!("Supported operations: Add, Subtract, Multiply, Divide, PowerOf(^).");
    println!("Enter your arithmetic expression below:");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => println!("Error: {}", err),
                };
        }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is: {:?}", ast);

    Ok(ast::eval(ast)?)
}
