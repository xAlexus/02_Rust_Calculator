use std::io;

fn main() {
    println!("Simple Calculator");

    loop {
    println!("Please provide the first Number:");
    let num1 = get_input().parse::<f64>().unwrap();

    println!("Please provide the second Number:");
    let num2 = get_input().parse::<f64>().unwrap();

    println!("Choose your Operation (+, -, *, /):");
    let operator = get_input();

    let result= calculate(&num1, &num2, &operator);

    println!("The Solution is: {}", result);
    
        }

    
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read Line");

    input.trim().to_string()
}

fn calculate(num1: &f64, num2: &f64, operator: &str) -> f64 {
    if operator.trim() == "+" {
        num1 + num2
    } else if operator.trim() == "-" {
        num1 - num2
    } else if operator.trim() == "*" {
        num1 * num2
    } else if operator.trim() == "/" {
        if *num2 == 0.0 {panic!("Thats not allowed, you can't devide by 0")} else {
            num1 / num2
        }
       
    } else {
        panic!("Invalid Operator");
    }
}