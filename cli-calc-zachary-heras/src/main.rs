use clap::{Command, Arg};

fn main() {
    // I tried for hours to use the "derive" version of clap with no success
    let matches = Command::new("CLI Calculator")
        .version("0.1.0")
        .about("This program is a command line interface calculator that works with integers")
        .arg(
            Arg::new("operand1")
                .help("First operand")
                .short('1')
                .required(false)
                .requires("operator")
                .requires("operand1")
        )
        .arg(
            Arg::new("operator")
                .help("Operator")
                .short('o')
                .required(false)
                .requires("operand1")
                .requires("operand2")
        )
        .arg(
            Arg::new("operand2")
                .help("Second operand")
                .short('2')
                .required(false)
                .requires("operator")
                .requires("operand2")
        )
        .subcommand(
            Command::new("--about")
                .about("Displays details about author and program")
        )
        .get_matches();

    if matches.subcommand_matches("--about").is_some() {
        println!("About stuff here.")
    }

    // I do not understand why ::<String> is needed
    // If it is not included, cargo says that clone trait is needed, which is not possible with
    // &str. So, I am not sure how I can convert a argument to a &str directly
    // What is unwrap()?
    // How can I assign optionally
    let operand1_str: &str = matches.get_one::<String>("operand1").unwrap();
    let operator_str: &str = matches.get_one::<String>("operator").unwrap();
    let operand2_str: &str = matches.get_one::<String>("operand2").unwrap();

    let operand1: i32 = operand1_str.parse().expect("Invalid operand; Must be i32");
    let operand2: i32 = operand2_str.parse().expect("Invalid operand; Must be i32");

    let result: i32 = match operator_str {
        "+" | "add" => operand1 + operand2,
        "-" | "sub" => operand1 - operand2,
        "*" | "mul" => operand1 * operand2,
        "/" | "div" => {
            if operand2 == 0 {
                panic!("Division by zero");
            }
            operand1 / operand2
        }
        _ => {
            eprintln!("Invalid operator: {}", operator_str);
            return;
        }
    };

    println!("{}", result);
}
