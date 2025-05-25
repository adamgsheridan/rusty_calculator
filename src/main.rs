use std::io;

struct Calculator {
    data: f64,
}

impl Calculator {
    fn new() -> Self {
        Calculator {data: 0.0}
    }

    fn add(&mut self, num1: f64, num2: f64) -> f64 {
        self.data = num1 + num2;
        self.data
    }

    fn subtract(&mut self, num1: f64, num2: f64) -> f64 {
        self.data = num1 - num2;
        self.data
    }

    fn multiply(&mut self, num1: f64, num2: f64) -> f64 {
        self.data = num1 * num2;
        self.data
    }

    fn divide(&mut self, num1: f64, num2: f64) -> f64 {
        self.data = num1 / num2;
        self.data
    }

    fn get_last_answer(&self) -> f64 {
        self.data
    }

    fn clear(&mut self) -> f64 {
        self.data = 0.0;
        self.data
    }
}

fn get_number() -> f64{
    loop {
        println!("Please enter a floating-point number:");
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<f64>() {
                return num;
            }
        }

        println!("Invalid input. Please try again.");
    }
}

fn main() {
    println!("Welcome to the Rusty Calculator!");
    let mut calc = Calculator::new();
    
    loop {
        print!("Please choose the operator you wish to use ");
        println!("or type \"no\" to exit the program: ");

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let escape: String = line.trim().parse().expect("Please enter a valid number");
        if escape == "+" {
            let num1: f64 = get_number();
            let num2:f64 = get_number();
            println!("\nThe sum of the two numbers is {}\n", calc.add(num1, num2));
        }
        else if escape == "-" {
            let num1: f64 = get_number();
            let num2:f64 = get_number();
            println!("\nThe difference of {num1} and {num2} is {}\n", calc.subtract(num1, num2));
        }
        else if escape == "*" {
            let num1: f64 = get_number();
            let num2:f64 = get_number();
            println!("\nThe product of the two numbers is {}\n", calc.multiply(num1, num2));
        }
        else if escape == "/" {
            let num1: f64 = get_number();
            let num2:f64 = get_number();
            println!("\nThe quotient of {num1} divided by {num2} is {}\n", calc.divide(num1, num2));
        }
        else if escape == "history" {
            println!("The last answer is {}", calc.get_last_answer());
        }
        else if escape == "clear" {
            calc.clear();
            println!("The last entry was erased.");
        }
        else if escape == "no" {
            break;
        }
        else {
            println!("Invalid entry. Try again!")
        }
    }
    println!("Last answer: {}", calc.get_last_answer());
}