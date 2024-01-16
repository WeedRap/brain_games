use crate::cli::get_user_input;
use crate::consts::AMOUNT_OF_ROUNDS;

pub fn run_game<F>(instruction: &str, get_question_and_answer: F)
where
    F: Fn() -> (String, String),
{
    println!("Welcome to the Brain Games!");
    let user_name: String = get_user_input("May I have your name? ");
    println!("Hello, {}!", user_name);
    println!("{}", instruction);

    for _ in 0..AMOUNT_OF_ROUNDS {
        let (question, right_answer) = get_question_and_answer();
        println!("Question: {}", question);

        let user_answer = get_user_input("Your answer: ");

        if user_answer == right_answer {
            println!("Correct!");
        } else {
            println!(
                "'{}' is wrong answer ;(. Correct answer is '{}'.\n\
                Let's try again, {}!",
                user_answer, right_answer, user_name
            );
            std::process::exit(1);
        }
    }
    println!("Congratulations, {}!", user_name)
}
