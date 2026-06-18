use clap::{Parser, Subcommand};
use serde_json::json;
use std::io::{self, Write};

mod lib;
use lib::processor::process_data;

#[derive(Parser)]
#[command(name = "my-cli")]
#[command(about = "Cross-language CLI tool written in Rust", long_about = None)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "false")]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Process input data
    Process {
        /// Input data to process
        #[arg(value_name = "DATA")]
        data: String,
    },

    /// Echo input
    Echo {
        /// Message to echo
        #[arg(value_name = "MESSAGE")]
        message: String,
    },

    /// Interactive mode
    Interactive,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Process { data } => {
            match process_data(&data) {
                Ok(result) => {
                    if args.json {
                        println!("{}", json!({ "status": "success", "result": result }));
                    } else {
                        println!("Result: {}", result);
                    }
                }
                Err(e) => {
                    if args.json {
                        eprintln!("{}", json!({ "status": "error", "message": e.to_string() }));
                    } else {
                        eprintln!("Error: {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }

        Commands::Echo { message } => {
            if args.json {
                println!("{}", json!({ "status": "success", "echo": message }));
            } else {
                println!("{}", message);
            }
        }

        Commands::Interactive => {
            interactive_mode();
        }
    }
}

fn interactive_mode() {
    println!("Entering interactive mode. Type 'exit' to quit.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            println!("Goodbye!");
            break;
        }

        match process_data(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
