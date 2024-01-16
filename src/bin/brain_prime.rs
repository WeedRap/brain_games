use brain_games::{
    consts::PRIME_INSTRUCTION, engine::run_game, games::prime::get_num_and_prime_answer,
};

fn main() {
    run_game(PRIME_INSTRUCTION, get_num_and_prime_answer);
}
