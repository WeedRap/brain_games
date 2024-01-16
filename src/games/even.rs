use crate::utils::get_rand_num;

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn get_num_and_even_answer() -> (String, String) {
    let problem_num = get_rand_num(1, 10);
    let right_answer = if is_even(problem_num) { "yes" } else { "no" };

    (problem_num.to_string(), right_answer.to_string())
}
