use crate::utils::get_rand_num;

fn is_prime(num: i32) -> bool {
    if num < 2 {
        false;
    }
    let prime_seq = (num as f64).sqrt() as i32;
    for i in 2..=prime_seq {
        if num % i == 0 {
            false;
        }
    }
    true
}

pub fn get_num_and_prime_answer() -> (String, String) {
    let num = get_rand_num(1, 10);
    let answer = if is_prime(num) { "yes" } else { "no" };

    (num.to_string(), answer.to_string())
}
