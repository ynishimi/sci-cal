mod menu;
use std::{io, io::Write};

fn main() {
    // casting i64 to f64
    let mut result: f64 = 0 as f64;
    loop {
        println!("Current Result: {result}");
        println!("{}", menu::export_menu());
        let menu_input = menu_selection();
        match menu_input {
            0 => exit(),
            1 => addition(&mut result),
            2 => subtraction(&mut result),
            _ => {
                println!("Error: Invalid selection!");
            },
        };
    }
}   

fn menu_selection() -> i8 {
    print!{"Enter Menu Selection: "};
    io::stdout().flush().unwrap();
    let mut menu_input = String::new();
    match io::stdin().read_line(&mut menu_input) {
        Ok(_) => {
            return menu_input.trim().parse().expect("Error: Invalid selection!");
        },
        Err(error) => panic!("{}", error),
    };
}

fn enter_operand(result:  &f64) -> f64 {
    io::stdout().flush().unwrap();
    let mut operand = String::new();
    match io::stdin().read_line(&mut operand) {
        Ok(_) => {
            match operand.trim() {
                "RESULT" => return *result,
                // Other than "RESULT"
                _ => {
                    let operand: f64 = operand.trim().parse().expect("Error: Invalid input!");
                    return operand
                }
            }
        }
        Err(error) => panic!("{}", error),
    };
}

fn enter_first_operand(result: & f64) -> f64 {
    print!("Enter first operand: ");
    io::stdout().flush().unwrap();
    enter_operand(result)
}

fn enter_second_operand(result: & f64) -> f64 {
    print!("Enter second operand: ");
    io::stdout().flush().unwrap();
    enter_operand(result)
}

// 0. Exit Program
fn exit() {
    println!("Thanks for using this calculator. Goodbye!");
    // Ends the program successfully
    std::process::exit(0);
}

fn operation<T>(result: &mut f64, calc: T)
where
    T: Fn(f64, f64) -> f64,
{
    let first_operand = enter_first_operand(&result);
    let second_operand = enter_second_operand(&result);
    // add * to change the value of result
    *result = calc(first_operand, second_operand);
}

// 1. Addition
fn addition(result: &mut f64) {
    operation(result, |a, b| a + b);
}

// // 2. Subtraction
fn subtraction(result: &mut f64) {
    operation(result, |a, b| a - b);
}

// // 3. Multiplication
// fn multiplication(result: f64) {
//     let first_operand = enter_first_operand(result);
//     let second_operand = enter_second_operand(result);
//     let result = first_operand * second_operand;
//     println!("{}", result);
// }
// // 4. Division
// fn division() {

// }
// // 5. Exponentiation
// fn exponentiation() {

// }
// // 6. Logarithm
// fn lograithm() {

// }
// // 7. Display Average
// fn display_average() {

// }