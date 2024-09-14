use colored::Colorize;
use miette::{IntoDiagnostic, Report, Result};
use polenta::{Polenta, PolentaUtilExt};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

const CMD_HELP: &str = "help";
const CMD_EXIT: &str = "exit";
const CMD_RESET: &str = "reset";

fn main() -> Result<()> {
    println!("Welcome to Polenta!");
    println!(
        "Type {} to quit, or see {} for all commands.",
        CMD_EXIT.yellow(),
        CMD_HELP.yellow()
    );
    let mut polenta = Polenta::<F>::new();
    let mut rl = DefaultEditor::new().into_diagnostic()?;

    let prompt_line = format!("{}", "> ".green());
    loop {
        match rl.readline(&prompt_line) {
            Ok(line) => match line.as_str() {
                "" => {
                    // do nothing
                }
                CMD_HELP => {
                    println!("Polenta is a simple language for polynomial manipulation.");
                    println!("{:<7}show this help message", CMD_HELP.yellow());
                    println!("{:<7}exit the program", CMD_EXIT.yellow());
                    println!("{:<7}reset symbols & history", CMD_RESET.yellow());
                }
                CMD_EXIT => {
                    println!("bye!");
                    break;
                }
                CMD_RESET => {
                    polenta = Polenta::<F>::new();
                    let _ = rl.clear_history();
                    println!("Symbol table reset.");
                }
                // TODO:!!!
                "field" => {
                    println!("Order: {}", F::ORDER);
                    todo!("change underlying field if arg is given, otherwise print name");
                }

                _ => {
                    // add ; to the input
                    let line_sanitized = format!("{};", line);
                    let input = line_sanitized.as_str();
                    let _ = rl.add_history_entry(input);

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
