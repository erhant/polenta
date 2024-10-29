use colored::Colorize;
use miette::{IntoDiagnostic, Report, Result};
use polenta::{Polenta, PolentaFields};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

const CMD_HELP: &str = "help";
const CMD_EXIT: &str = "exit";
const CMD_RESET: &str = "reset";
const CMD_FIELD: &str = "field";

const WELCOME_BANNER: &str = r#"
              _            _
  _ __   ___ | | ___ _ __ | |_ __ _
 | '_ \ / _ \| |/ _ \ '_ \| __/ _` |
 | |_) | (_) | |  __/ | | | || (_| |
 | .__/ \___/|_|\___|_| |_|\__\__,_|
 |_|
"#;

fn main() -> Result<()> {
    println!(
        "{}\n(v{})",
        WELCOME_BANNER.green().bold(),
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "Type {} to quit, or see {} for all commands.",
        CMD_EXIT.yellow(),
        CMD_HELP.yellow()
    );

    // default is Goldilocks
    let mut polenta = PolentaFields::default();
    let mut rl = DefaultEditor::new().into_diagnostic()?;

    let prompt_line = format!("{}", "> ".green());
    loop {
        match rl.readline(&prompt_line) {
            // TODO: use clap here
            Ok(line) => match line.as_str() {
                "" => {
                    // do nothing
                }
                CMD_HELP => {
                    let _ = rl.add_history_entry(CMD_HELP);
                    println!("Polenta is a toy language for polynomial manipulation.");
                    println!("{:<7}show this help message", CMD_HELP.yellow());
                    println!("{:<7}exit the program", CMD_EXIT.yellow());
                    println!("{:<7}reset symbols", CMD_RESET.yellow());
                    println!("");
                    println!("Use arrow keys for command history.");
                }
                CMD_EXIT => {
                    println!("bye!");
                    break;
                }
                CMD_RESET => {
                    polenta.reset();
                    println!("Cleared symbols.");
                }
                CMD_FIELD => {
                    polenta = PolentaFields::Mersenne31(Polenta::new());
                }
                _ => {
                    // ensure input ends with `;`
                    let line_sanitized = format!("{};", line.trim_end_matches(';'));
                    let input = line_sanitized.as_str();
                    let _ = rl.add_history_entry(input);

                    // process input
                    let result = polenta.interpret(input);
                    match result {
                        Ok(result) => {
                            println!("{}", result.blue());
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
