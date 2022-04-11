use super::*;

// Basic test to check bmi is properly calculated
#[test]
fn bmi_calc_test() {
    let test_input = BmiInput {
        h_feet: 5,
        h_inch: 3,
        weight: 125.0,
    };

    let test_output = BmiResult {
        bmi: 22.7,
        category: "Normal".to_string(),
    };

    let test_result: BmiResult = bmi_calculator(test_input);

    assert_eq!(test_result, test_output);
}

/* 
Tests to check classification of BMI
N x 1 with a Shift of 0.1
Underweight: 
On 18.5, Off 18.4
Normal:
On 18.5, Off 18.4
Off 25, On 24.9
Overweight:
On 25, Off 24.9
Off 30, On 29.9
Obese:
On 30, Off 29.9
*/
#[test]
fn underweight_upper_on() {
    let test_result: String = bmi_classify(18.5);
    assert_ne!(test_result, "Underweight");
}

#[test]
fn underweight_upper_off() {
    let test_result: String = bmi_classify(18.4);
    assert_eq!(test_result, "Underweight");
}

#[test]
fn normal_lower_on() {
    let test_result: String = bmi_classify(18.5);
    assert_eq!(test_result, "Normal");
}

#[test]
fn normal_lower_off() {
    let test_result: String = bmi_classify(18.4);
    assert_ne!(test_result, "Normal");
}

#[test]
fn normal_upper_on() {
    let test_result: String = bmi_classify(25.0);
    assert_ne!(test_result, "Normal");
}

#[test]
fn normal_upper_off() {
    let test_result: String = bmi_classify(24.9);
    assert_eq!(test_result, "Normal");
}

#[test]
fn overweight_lower_on() {
    let test_result: String = bmi_classify(25.0);
    assert_eq!(test_result, "Overweight");
}

#[test]
fn overweight_lower_off() {
    let test_result: String = bmi_classify(24.9);
    assert_ne!(test_result, "Overweight");
}

#[test]
fn overweight_upper_on() {
    let test_result: String = bmi_classify(30.0);
    assert_ne!(test_result, "Overweight");
}

#[test]
fn overweight_upper_off() {
    let test_result: String = bmi_classify(29.9);
    assert_eq!(test_result, "Overweight");
}

#[test]
fn obese_lower_on() {
    let test_result: String = bmi_classify(30.0);
    assert_eq!(test_result, "Obese");
}

#[test]
fn obese_lower_off() {
    let test_result: String = bmi_classify(29.9);
    assert_ne!(test_result, "Obese");
}