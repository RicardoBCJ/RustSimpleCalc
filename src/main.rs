use std::io;

fn main() {
    println!("Basic Calculator in Rust!");

    loop {
        println!("Enter the first number:");
        let mut first_number = String::new();
        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read number");
        let first_number: f64 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number.");
                continue;
            }
        };

        println!("Enter the second number:");
        let mut second_number = String::new();
        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read number");
        let second_number: f64 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number.");
                continue;
            }
        };

        println!("Enter an operator (+, -, *, /, **, root)");
        let mut operator = String::new();
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line");
        let operator = operator.trim();

        let result = match operator {
            "+" => first_number + second_number,
            "-" => first_number - second_number,
            "*" => first_number * second_number,
            "/" => {
                if second_number != 0.0 {
                    first_number / second_number
                } else {
                    print!("Cannot divide by zero, the world will explode!");
                    continue;
                }
            }
            "**" => first_number.powf(second_number),
            "root" => first_number.powf(1.0 / second_number),
            _ => {
                println!("Invalid operator, please use +, -, *, /, **. root");
                continue;
            }
        };

        println!("Result:{}", result);
        break;
    }
}
