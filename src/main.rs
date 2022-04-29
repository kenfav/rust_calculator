use eval::Value::Number;
use std::process::abort;
use std::io;
use eval::eval;
use std::fmt;

fn main() {
    let option: String = calculator_menu();
    calculator_select_option(option);

}
// Read a number from the user
// Read an operator from the user 
// Read an second number from the user
// Display the result
// create a menu with calculate and exit

fn calculator_menu() -> String {
    println!("Welcome to the calculator app. Let's start. Digit a number: ");
    println!("(1) Calculate:");
    println!("(2) Quit");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    if input.trim() != "1" && input.trim() != "2" {
        println!("Please digit a valid option.");
        calculator_menu();
    }
    input.trim().to_string()
}

fn calculator_select_option(option: String) {
    if option == "1" {
        calculator_start();
    } else {
        abort(); 
    }    
}
        
fn calculator_start() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();
    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1);
    println!("Enter the operator: ");
    io::stdin().read_line(&mut operator);
    println!("Enter the second number");
    io::stdin().read_line(&mut num2);
    calculate(num1.trim().to_string(), num2.trim().to_string(), operator.trim().to_string());
}

fn calculate(num1: String, num2: String, operator: String) {
    let expression: String = format!("{}{}{}", num1, operator, num2);
    let result = eval(&expression);
    match result {
        Ok(Number(i)) => println!("Result: {}",i),
        Err(_) => calculator_select_option("1".to_string()),
        other => {
            println!("Invalid numbers or operator. Try again.");
            calculator_select_option("1".to_string())
        }, 
    }
    let option: String = calculator_menu();
    calculator_select_option(option);
}
