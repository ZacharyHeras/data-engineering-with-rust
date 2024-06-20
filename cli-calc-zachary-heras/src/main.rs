use clap::{Arg, Command};

fn calculate(operand1: i32, operand2: i32, operator_str: &str) -> i32 {
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
            // panic here or return none? I chose to panic
            panic!("Invalid operator: {}", operator_str);
        }
    };

    return result;
}

fn main() {
    let matches = Command::new("CLI Calculator")
        .version("0.1.0")
        .about("This program is a command line interface calculator that works with integers")
        .arg(
            Arg::new("operand1")
                .help("First operand")
                .short('1')
                .required(false)
                .requires("operator")
                .requires("operand1"),
        )
        .arg(
            Arg::new("operator")
                .help("Operator")
                .short('o')
                .required(false)
                .requires("operand1")
                .requires("operand2"),
        )
        .arg(
            Arg::new("operand2")
                .help("Second operand")
                .short('2')
                .required(false)
                .requires("operator")
                .requires("operand2"),
        )
        .subcommand(Command::new("--about").about("Displays details about author and program"))
        .get_matches();

    if matches.subcommand_matches("--about").is_some() {
        println!("Made by Zachary Heras. This program is an integer CLI calculator.")
    } else {
        let operand1_str: &str = matches.get_one::<String>("operand1").unwrap();

        let operand2_str: &str = matches.get_one::<String>("operand2").unwrap();

        let operator_str: &str = matches.get_one::<String>("operator").unwrap();

        let operand1: i32 = operand1_str.parse().expect("Invalid operand; Must be i32");
        let operand2: i32 = operand2_str.parse().expect("Invalid operand; Must be i32");

        let result = calculate(operand1, operand2, operator_str);

        println!("{}", result);
    }
}


#[test]
fn test_calculate_add() {
    assert!(calculate(2, 5, "add") == 7);
    assert!(calculate(2, 5, "+") == 7);
}

#[test]
fn test_calculate_sub() {
    assert!(calculate(2, 5, "sub") == -3);
    assert!(calculate(2, 5, "-") == -3);
}

#[test]
fn test_calculate_mul() {
    assert!(calculate(2, 5, "mul") == 10);
    assert!(calculate(2, 5, "*") == 10);
}

#[test]
fn test_calculate_div() {
    assert!(calculate(2, 5, "div") == 0);
    assert!(calculate(2, 5, "/") == 0);
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_calculate2() {
    calculate(4, 0, "div");
    calculate(4, 0, "/");
}

#[test]
#[should_panic(expected = "Invalid operator: hello world")]
fn test_calculate3() {
    calculate(1, 2, "hello world");
}
