use crate::{consts::PROGRESSION_LENGTH, utils::get_rand_num};

pub fn get_progression_and_missed_num() -> (String, String) {
    let start = get_rand_num(1, 10);
    let step = get_rand_num(1, 10);

    let mut progression = Vec::new();
    for i in 0..PROGRESSION_LENGTH {
        let value: String = format!("{}", start + step * i);
        progression.push(value);
    }
    let missed_ind = get_rand_num(1, PROGRESSION_LENGTH - 1) as usize;
    let missed_num = progression[missed_ind].clone();
    progression[missed_ind] = String::from("..");

    (progression.join(" "), missed_num)
}
