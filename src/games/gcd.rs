use crate::utils::get_rand_num;
use num::integer::gcd;

pub fn get_nums_pair_and_gcd() -> (String, String) {
    let (num1, num2) = (get_rand_num(1, 10), get_rand_num(1, 10));

    let nums_pair = format!("{} {}", num1, num2);
    let gcd = gcd(num1, num2);

    (nums_pair, gcd.to_string())
}
