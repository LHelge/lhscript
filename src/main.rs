use clap::Parser;
use std::{
    fs::read_to_string,
    io::{stdin, BufRead},
    path::PathBuf,
};

mod context;
use context::Context;

mod errors;
use errors::ScriptError;

mod scanner;
use scanner::*;

mod token;
use token::*;

mod ast;
use ast::*;

mod parser;


#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Filename to run
    file: Option<String>,

    /// Run in interactive mode after executing file
    #[arg(short, long)]
    interactive: bool,
}

fn main() {
    let args = Args::parse();


        // Temporary
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Unary(UnaryExpression {
                operator: Token::Minus,
                right: Box::new(Expression::Literal(LiteralExpression { 
                    literal: Token::Number(123f64) 
                })),
            })),
            operator: Token::Star,
            right: Box::new(Expression::Grouping(GroupingExpression { 
                group: Box::new(Expression::Literal(LiteralExpression { 
                    literal: Token::Number(45.67f64),
                })),
            })),
        });
    
        let printer = AstPrinter;
        let exp = printer.print(expr).unwrap();
        println!("AST-test: {}", exp);
    
        let tokens = "1 * (2 + 3)".tokens().unwrap();
        println!("Tokens: {:?}", tokens);
        let mut parser = parser::Parser::new(tokens);
        let expr = parser.parse().unwrap();
        println!("Expression: {:?}", expr);
        let exp = printer.print(expr).unwrap();
        println!("AST-printer: {}", exp);


    let mut context = Context::new();

    if let Some(file) = args.file {
        println!("Running file {}, interactive={}", file, args.interactive);
        context = run_file(PathBuf::from(file), context).expect("Error");

        if args.interactive {
            _ = run_prompt(context)
        }
    } else {
        println!("Running prompt:");
        _ = run_prompt(context).expect("Error");
    }




}

fn run_file(path: PathBuf, mut context: Context) -> Result<Context, ScriptError> {
    let script = read_to_string(path)?;

    context = run(&script, context)?;

    Ok(context)
}

fn run_prompt(mut context: Context) -> Result<Context, ScriptError> {
    let mut buffer = String::new();
    let mut stdin = stdin().lock();

    loop {
        print!(">>> ");

        stdin.read_line(&mut buffer)?;

        context = run(&buffer, context)?;

        if context.should_exit {
            break;
        }
    }

    Ok(context)
}

fn run(script: &str, mut context: Context) -> Result<Context, ScriptError> {
    // TODO: parse

    let tokens = script.tokens()?;

    for (index, token) in tokens.into_iter().enumerate() {
        println!("{}: {:?}", index, token);
    }

    context.should_exit = true;

    Ok(context)
}
