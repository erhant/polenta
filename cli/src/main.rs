use colored::Colorize;
use miette::{IntoDiagnostic, Report, Result};
use polenta::{Polenta, PolentaUtilExt};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

fn main() -> Result<()> {
    println!("Welcome to Polenta!");
    println!("Type '.exit' to quit.");
    let mut polenta = Polenta::<F>::new();
    let mut rl = DefaultEditor::new().into_diagnostic()?;

    let prompt_line = format!("{}", "> ".green());
    loop {
        match rl.readline(&prompt_line) {
            Ok(line) => match line.as_str() {
                ".help" => {
                    println!("Polenta is a simple language for polynomial manipulation.");
                    println!("{:<7}show this help message", ".help".yellow());
                    println!("{:<7}exit the program", ".exit".yellow());
                    println!("{:<7}reset the symbol table", ".reset".yellow());
                }
                ".exit" => {
                    println!("bye!");
                    break;
                }
                ".reset" => {
                    polenta = Polenta::<F>::new();
                    let _ = rl.clear_history();
                    println!("Symbol table reset.");
                }
                // TODO:!!!
                ".field" => {
                    println!("Order: {}", F::ORDER);
                    todo!("change underlying field if arg is given, otherwise print name");
                }
                _ => {
                    let input = line.trim();
                    let _ = rl.add_history_entry(input);

                    // empty string check
                    if input.is_empty() {
                        continue;
                    }

                    // process input
                    let result = polenta.interpret(input);
                    match result {
                        Ok(polys) => {
                            println!("{}", Polenta::poly_print(polys.last().unwrap()).blue());
                        }
                        Err(e) => {
                            println!("{:?}", Report::from(e));
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("bye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
