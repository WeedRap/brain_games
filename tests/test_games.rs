use regex::Regex;

use brain_games::consts::{MATH_SIGNS, PROGRESSION_LENGTH};
use brain_games::games::{
    calc::get_math_expression_and_result, even::get_num_and_even_answer,
    gcd::get_nums_pair_and_gcd, prime::get_num_and_prime_answer,
    progression::get_progression_and_missed_num,
};

const YES_AND_NO_ANSWERS: [&str; 2] = ["yes", "no"];
const NUM_REGEX: &str = r"^-?\d+$";

#[test]
fn test_get_num_and_even_answer() {
    let (problem_num, answer) = get_num_and_even_answer();

    let num_regex = Regex::new(NUM_REGEX).unwrap();
    assert!(num_regex.is_match(&problem_num));
    assert!(YES_AND_NO_ANSWERS.contains(&&*answer));
}

#[test]
fn test_get_math_expression_and_result() {
    let (expression, result) = get_math_expression_and_result();

    let math_expression_regex =
        Regex::new(&format!(r"^\d+\s[({})]\s\d+$", MATH_SIGNS.join("\\"))).unwrap();
    assert!(math_expression_regex.is_match(&expression));

    let num_regex = Regex::new(NUM_REGEX).unwrap();
    assert!(num_regex.is_match(&result));
}

#[test]
fn test_get_nums_pair_and_gcd() {
    let (nums_pair, gcd) = get_nums_pair_and_gcd();

    let nums_pair_regex = Regex::new(r"^\d+\s\d+$").unwrap();
    assert!(nums_pair_regex.is_match(&nums_pair));

    let num_regex = Regex::new(NUM_REGEX).unwrap();
    assert!(num_regex.is_match(&gcd));
}

#[test]
fn test_get_num_and_prime_answer() {
    let (problem_num, answer) = get_num_and_prime_answer();

    let nums_pair_regex = Regex::new(NUM_REGEX).unwrap();

    assert!(nums_pair_regex.is_match(&problem_num));
    assert!(YES_AND_NO_ANSWERS.contains(&&*answer));
}

#[test]
fn test_get_progression_and_missed_num() {
    let (progression, missed_num) = get_progression_and_missed_num();

    let progression_regex = Regex::new(&format!(
        r"(?:(\b|\.\.\s)\d+\b\s?(\.\.)?){{{}}}",
        PROGRESSION_LENGTH - 1
    ))
    .unwrap();
    assert!(progression_regex.is_match(&progression));

    let num_regex = Regex::new(NUM_REGEX).unwrap();
    assert!(num_regex.is_match(&missed_num));
}
