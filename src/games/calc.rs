use crate::consts::MATH_SIGNS;
use crate::utils::get_rand_num;
use rand::seq::SliceRandom;

fn calculate(num1: u8, num2: u8, math_sign: &str) -> i16 {
    match math_sign {
        "+" => (num1 + num2) as i16,
        "-" => num1 as i16 - num2 as i16,
        "*" => (num1 * num2) as i16,
        _ => panic!("Unsupported math operation"),
    }
}

pub fn get_math_expression_and_result() -> (String, String) {
    let num1: u8 = get_rand_num(1, 10);
    let num2: u8 = get_rand_num(1, 10);
    let math_sign: &str = MATH_SIGNS.choose(&mut rand::thread_rng()).unwrap();

    let expression: String = format!("{} {} {}", num1, math_sign, num2);
    let result: i16 = calculate(num1, num2, math_sign);

    (expression, result.to_string())
}
