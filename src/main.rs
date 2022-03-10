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
    unimplemented!();
}

fn bmi_calculator(input: BmiInput) -> BmiResult {
    unimplemented!();
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests;