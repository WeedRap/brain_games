use rand::Rng;

fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }
    let prime_seq: i32 = (num as f64).sqrt() as i32;
    for i in 2..=prime_seq {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn get_num_and_prime_answer() -> (String, String) {
    let num: i32 = rand::thread_rng().gen_range(1..10);
    let answer = if is_prime(num) { "yes" } else { "no" };

    (num.to_string(), answer.to_string())
}
