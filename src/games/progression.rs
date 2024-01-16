use crate::{consts::PROGRESSION_LENGTH, utils::get_rand_num};

pub fn get_progression_and_missed_num() -> (String, String) {
    let start: u8 = get_rand_num(1, 10);
    let step: u8 = get_rand_num(1, 10);

    let mut progression: Vec<String> = Vec::new();
    for i in 0..PROGRESSION_LENGTH {
        let value: String = format!("{}", start + step * i);
        progression.push(value);
    }
    let missed_ind: u8 = get_rand_num(0, PROGRESSION_LENGTH - 1);
    let missed_num: String = progression[missed_ind as usize].clone();
    progression[missed_ind as usize] = String::from("..");

    (progression.join(" "), missed_num)
}
