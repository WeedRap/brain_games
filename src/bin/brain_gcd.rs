use brain_games::{consts::GCD_INSTRUCTION, engine::run_game, games::gcd::get_nums_pair_and_gcd};

fn main() {
    run_game(GCD_INSTRUCTION, get_nums_pair_and_gcd)
}
