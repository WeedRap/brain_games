use crate::consts::MATH_SIGNS;
use crate::utils::get_rand_num;
use rand::seq::SliceRandom;

fn calculate(num1: i32, num2: i32, math_sign: &str) -> i32 {
    match math_sign {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        _ => panic!("Unsupported math operation"),
    }
}

pub fn get_math_expression_and_result() -> (String, String) {
    let num1 = get_rand_num(1, 10);
    let num2 = get_rand_num(1, 10);
    let math_sign: &str = MATH_SIGNS.choose(&mut rand::thread_rng()).unwrap();

    let expression = format!("{} {} {}", num1, math_sign, num2);
    let result = calculate(num1, num2, math_sign);

    (expression, result.to_string())
}
