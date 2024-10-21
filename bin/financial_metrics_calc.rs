// example of the ROI forumla
use std::io::{self};

fn calculate_roi(current_value: f64, inital_value: f64) -> f64 {
    (current_value - inital_value) / inital_value
}

fn calculate_npv(initial_investment: f64, cash_flow: Vec<f64>, discount_rate: f64) -> f64 {
    let mut npv = -initial_investment;
    for (index, &cash_flow) in cash_flow.iter().enumerate() {
        npv += cash_flow / (1.0 + discount_rate).powi(index as i32 + 1);
    }
    npv
}

//helper function to get single fload input
fn get_float_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
    }
}

// Helper function to get a vector of floats
fn get_float_vector_input(prompt: &str) -> Vec<f64> {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let numbers: Vec<f64> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if !numbers.is_empty() {
            return numbers;
        } else {
            println!("Please enter at least one number!");
        }
    }
}

fn main() {
    println!("Finalcial Metrics Calculator:");

    //ROI calculation
    let inital_value = get_float_input("Enter the initial value: ");
    let current_value = get_float_input("Enter the current value: ");
    let roi = calculate_roi(current_value, inital_value);
    println!("The ROI is: {:.2}%", roi);

    //NPV calculation
    let inial_investment = get_float_input("Enter the initial investment: ");
    let cash_flow = get_float_vector_input("Enter the cash flow (separated by spaces): ");
    let discount_rate = get_float_input("Enter the discount rate: ");
    let npv = calculate_npv(inial_investment, cash_flow, discount_rate);
    println!("The NPV is: {:.2}", npv);
}
