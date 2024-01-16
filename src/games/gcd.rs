use crate::utils::get_rand_num;

fn calculate_gcd(mut num1: u8, mut num2: u8) -> u8 {
    while num2 != 0 {
        let tmp = num2;
        num2 = num1 % num2;
        num1 = tmp;
    }
    num1
}

pub fn get_nums_pair_and_gcd() -> (String, String) {
    let num1: u8 = get_rand_num(1, 10);
    let num2: u8 = get_rand_num(1, 10);

    let nums_pair: String = format!("{} {}", num1, num2);
    let gcd: u8 = calculate_gcd(num1, num2);

    (nums_pair, gcd.to_string())
}
