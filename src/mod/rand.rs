use rand::Rng;

pub fn generate_random_position(lower_limit: usize, higher_limit: usize) -> usize {
    let random_num = rand::thread_rng().gen_range(lower_limit..=higher_limit);
    
    random_num
}