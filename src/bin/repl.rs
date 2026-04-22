use colored::Colorize;
use miette::{IntoDiagnostic, Report, Result};
use polenta::PolentaInstance;
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
    let mut polenta = PolentaInstance::default();
    let mut rl = DefaultEditor::new().into_diagnostic()?;

    let prompt_line = format!("{}", "> ".green());
    loop {
        match rl.readline(&prompt_line) {
            Ok(line) => match line.as_str() {
                "" => {}
                CMD_HELP => {
                    let _ = rl.add_history_entry(CMD_HELP);

                    println!("Polenta is a simple language for polynomial manipulation.");
                    println!("{:<14}show this help message", CMD_HELP.yellow());
                    println!("{:<14}exit the program", CMD_EXIT.yellow());
                    println!("{:<14}reset symbols", CMD_RESET.yellow());
                    println!(
                        "{:<14}show current field or switch by name",
                        CMD_FIELD.yellow()
                    );
                }
                CMD_EXIT => {
                    println!("bye!");
                    break;
                }
                CMD_RESET => {
                    polenta = PolentaInstance::new_from_name(polenta.name())
                        .expect("current field must be in catalog");
                    println!("Symbol table reset.");
                }
                line if line.starts_with(CMD_FIELD) => {
                    let _ = rl.add_history_entry(line);
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if parts.len() == 1 {
                        let current = polenta.name();
                        println!(
                            "Current field: {} (modulus: {})",
                            current.yellow(),
                            polenta.modulus()
                        );
                        println!("Available fields:");
                        for (name, factory) in PolentaInstance::supported_fields() {
                            let marker = if *name == current { "*" } else { " " };
                            println!(
                                "  {} {} (modulus: {})",
                                marker,
                                name.blue(),
                                factory().modulus()
                            );
                        }
                        println!("Use {} to switch fields.", "field <name>".yellow());
                    } else if parts.len() == 2 {
                        let target = parts[1];
                        if target.eq_ignore_ascii_case(polenta.name()) {
                            println!("Already using {} field.", polenta.name().yellow());
                        } else {
                            match PolentaInstance::new_from_name(target) {
                                Some(new_polenta) => {
                                    let old_polenta = std::mem::replace(&mut polenta, new_polenta);
                                    polenta.migrate_symbols_from(&old_polenta);
                                    println!(
                                        "Switched to {} field (modulus: {}). Symbol table migrated.",
                                        polenta.name().yellow(),
                                        polenta.modulus()
                                    );
                                }
                                None => {
                                    println!(
                                        "Unknown field: {}. Available fields: {}",
                                        target.red(),
                                        PolentaInstance::supported_field_names().join(", ")
                                    );
                                }
                            }
                        }
                    } else {
                        println!(
                            "Usage: {} or {}",
                            CMD_FIELD.yellow(),
                            "field <name>".yellow()
                        );
                    }
                }

                _ => {
                    let line_sanitized = format!("{};", line);
                    let input = line_sanitized.as_str();
                    let _ = rl.add_history_entry(input);

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
