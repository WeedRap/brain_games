use rand::Rng;

pub fn get_rand_num(start: i32, stop: i32) -> i32 {
    rand::thread_rng().gen_range(start..=stop)
}
