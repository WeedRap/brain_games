use rand::Rng;

pub fn get_rand_num(start: u8, stop: u8) -> u8 {
    rand::thread_rng().gen_range(start..=stop)
}
