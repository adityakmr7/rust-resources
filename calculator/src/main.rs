use std::io;

// Define the Operation enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Implement the calculate function
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                println!("Warning: Division by zero is not allowed.");
                f64::NAN
            } else {
                a / b
            }
        }
    }
}

fn main() {
    // Helper function to read input
    fn read_input(prompt: &str) -> String {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }

    let num1: f64 = read_input("Enter the first number:")
        .parse()
        .expect("Invalid number");

    let op = read_input("Enter the operation (+, -, *, /):");

    let num2: f64 = read_input("Enter the second number:")
        .parse()
        .expect("Invalid number");

    // Match operation symbol to Operation enum
    let operation = match op.as_str() {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = calculate(operation);
    println!("Result: {}", result);
}
