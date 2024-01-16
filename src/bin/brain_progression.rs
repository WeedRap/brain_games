use brain_games::{
    consts::PROGRESSION_INSTRUCTION, engine::run_game,
    games::progression::get_progression_and_missed_num,
};

fn main() {
    run_game(PROGRESSION_INSTRUCTION, get_progression_and_missed_num)
}
