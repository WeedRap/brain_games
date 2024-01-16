use brain_games::{
    consts::CALC_INSTRUCTION, engine::run_game, games::calc::get_math_expression_and_result,
};

fn main() {
    run_game(CALC_INSTRUCTION, get_math_expression_and_result);
}
