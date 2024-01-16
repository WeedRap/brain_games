use brain_games::{
    consts::EVEN_INSTRUCTION, engine::run_game, games::even::get_num_and_even_answer,
};

fn main() {
    run_game(EVEN_INSTRUCTION, get_num_and_even_answer);
}
