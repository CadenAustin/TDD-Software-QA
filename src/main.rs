extern crate round;
use round::{round};

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
    println!("Hello, world!");
}

#[cfg(test)]
mod tests;