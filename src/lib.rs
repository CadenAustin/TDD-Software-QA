#[cfg(test)]
mod tests;
mod utils;

extern crate text_io;
extern crate round;
use text_io::read;
use round::round;
use std::io::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tdd-software-qa!");
}



pub struct BmiInput {
    h_feet: i32,
    h_inch: i32,
    weight: f64,
}

#[derive(PartialEq, Debug)]
pub struct BmiResult {
    bmi: f64,
    category: String,
}

fn bmi_classify(bmi: f64) -> String {
    match Some(bmi) {
        Some(x) if x < 18.5 => "Underweight".to_string(),
        Some(x) if x <= 24.9 => "Normal".to_string(),
        Some(x) if x <= 29.9 => "Overweight".to_string(),
        Some(x) if x >= 30.0 => "Obese".to_string(),
        None => panic!("Invalid BMI"),
        _ => panic!("Invalid BMI"),
    }
}

fn bmi_calculator(input: BmiInput) -> BmiResult {
    let kilos = input.weight * 0.45;
    let height_inchs = (input.h_feet * 12 + input.h_inch) as f64;
    let height_m = height_inchs * 0.025;
    let squared_height = height_m * height_m;

    let bmi = round(kilos / squared_height, 1);
    let category = bmi_classify(bmi);

    BmiResult {
        bmi: bmi,
        category: category,
    }
}

fn main() {
    println!("Welcome to the BMI Calculator!");
    while 1==1 {
        println!("Please provide the following information:");
        print!("\tHeight in Feet: ");
        std::io::stdout().flush().expect("Flush Failed");
        let h_feet: i32 = read!();
        print!("\tHeight in Inches: ");
        std::io::stdout().flush().expect("Flush Failed");
        let h_inch: i32 = read!();
        print!("\tWeight in Pounds: ");
        std::io::stdout().flush().expect("Flush Failed");
        let weight: f64 = read!();
        
        let input = BmiInput {
            h_feet: h_feet,
            h_inch: h_inch,
            weight: weight,
        };

        let output = bmi_calculator(input);

        println!("\nBMI = {}\nCategory = {}", output.bmi, output.category);
        println!("Type q to quit, otherwise hit enter");
        let exit_option: String = read!("{}\n");
        match exit_option.as_str() {
            "q" | "Q" => {break}
            _ => {}
        };
        println!("");
    }
}

