/*
By: <Your Name Here>
Date: 2026-10-06
Program Details: <Program Description Here>
*/

fn main() {
    println!("Welcome to Celsius Calculator");
    let mut input1 = String::new();
    println!("Please enter your temperature:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");

    let number1 = input1.trim().parse::<f32>();

    if let Ok(number1) = number1 {
        println!("Converted temperature is: {}", number1 / 30.0 - 2.0)
    }else {
        println!("You can't divide letters");
    }
}
