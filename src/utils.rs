use rand::{thread_rng, Rng};

pub fn generate_random_coordinates(x_bound: i32, y_bound: i32) -> (i32, i32) {
    let random_x = thread_rng().gen_range(0..x_bound);
    let random_y = thread_rng().gen_range(0..y_bound);

    (random_x, random_y)
}
