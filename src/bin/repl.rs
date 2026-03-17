use colored::Colorize;
use miette::{IntoDiagnostic, Report, Result};
use polenta::{FieldType, PolentaInstance};
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
 |_|   v{{VERSION}}
"#;

fn main() -> Result<()> {
    println!(
        "{}",
        WELCOME_BANNER
            .replace("{{VERSION}}", env!("CARGO_PKG_VERSION"))
            .green(),
    );
    println!(
        "Type {} to quit, or see {} for all commands.",
        CMD_EXIT.yellow(),
        CMD_HELP.yellow()
    );
    let mut polenta = PolentaInstance::new(FieldType::Babybear31);
    let mut rl = DefaultEditor::new().into_diagnostic()?;

    let prompt_line = format!("{}", "> ".green());
    loop {
        match rl.readline(&prompt_line) {
            Ok(line) => match line.as_str() {
                "" => {
                    // do nothing
                }
                CMD_HELP => {
                    let _ = rl.add_history_entry(CMD_HELP);

                    println!("Polenta is a simple language for polynomial manipulation.");
                    println!("{:<12}show this help message", CMD_HELP.yellow());
                    println!("{:<12}exit the program", CMD_EXIT.yellow());
                    println!("{:<12}reset symbols", CMD_RESET.yellow());
                    println!("{:<12}show current field or switch field", "field".yellow());
                }
                CMD_EXIT => {
                    println!("bye!");
                    break;
                }
                CMD_RESET => {
                    polenta = PolentaInstance::new(polenta.field_type());
                    println!("Symbol table reset.");
                }
                line if line.starts_with(CMD_FIELD) => {
                    let _ = rl.add_history_entry(line);
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if parts.len() == 1 {
                        // show current field and available fields
                        let current = polenta.field_type();
                        println!(
                            "Current field: {} (order: {})",
                            current.name().yellow(),
                            current.order()
                        );
                        println!("Available fields:");
                        for field_type in FieldType::all() {
                            let marker = if *field_type == current { "*" } else { " " };
                            println!("  {}{}", marker, field_type.name().blue());
                        }
                        println!("Use {} to switch fields.", "field <name>".yellow());
                    } else if parts.len() == 2 {
                        // switch to the specified field
                        let field_name = parts[1];
                        match FieldType::from_name(field_name) {
                            Some(new_field_type) => {
                                if new_field_type == polenta.field_type() {
                                    println!(
                                        "Already using {} field.",
                                        new_field_type.name().yellow()
                                    );
                                } else {
                                    let new_polenta = PolentaInstance::new(new_field_type);
                                    let old_polenta = std::mem::replace(&mut polenta, new_polenta);
                                    polenta.migrate_symbols_from(&old_polenta);
                                    println!(
                                        "Switched to {} field (order: {}). Symbol table migrated.",
                                        new_field_type.name().yellow(),
                                        new_field_type.order()
                                    );
                                }
                            }
                            None => {
                                println!(
                                    "Unknown field: {}. Available fields: {}",
                                    field_name.red(),
                                    FieldType::all()
                                        .iter()
                                        .map(|f| f.name())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                );
                            }
                        }
                    } else {
                        println!("Usage: {} or {}", "field".yellow(), "field <name>".yellow());
                    }
                }

                _ => {
                    // add ; to the input
                    let line_sanitized = format!("{};", line);
                    let input = line_sanitized.as_str();
                    let _ = rl.add_history_entry(input);

                    // process input
                    match polenta.interpret(input) {
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
