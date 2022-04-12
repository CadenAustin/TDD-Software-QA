#[cfg(test)]
mod tests;
mod utils;

extern crate text_io;
extern crate round;
use text_io::read;
use round::round;
use std::{io::prelude::*, borrow::Borrow};
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


#[wasm_bindgen]
pub struct BmiInput {
    h_feet: i32,
    h_inch: i32,
    weight: f64,
}

#[wasm_bindgen]
impl BmiInput {
    pub fn new(h_feet: i32, h_inch: i32, weight: f64) -> BmiInput {
        BmiInput { h_feet: h_feet, h_inch: h_inch, weight: weight }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub struct BmiResult {
    bmi: f64,
    category: String,
}

#[wasm_bindgen]
impl BmiResult {
    #[wasm_bindgen(getter)]
    pub fn bmi(&self) -> f64 {
        self.bmi
    }

    #[wasm_bindgen(getter)]
    pub fn category(&self) -> String {
        self.category.clone()
    }
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

#[wasm_bindgen]
pub fn bmi_calculator(input: BmiInput) -> BmiResult {
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
