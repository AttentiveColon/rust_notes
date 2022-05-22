#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use rand::{self, Rng};

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn return_rand() -> u32 {
    rand::thread_rng().gen_range(1..101)
}