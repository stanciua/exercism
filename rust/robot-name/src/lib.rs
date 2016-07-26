extern crate rand;

use std::collections::HashMap;
use rand::distributions::{IndependentSample, Range};

#[derive(Default)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: Robot::generate_robot_name() }
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn reset_name(&mut self) {
        self.name.clear();
        self.name = Robot::generate_robot_name();
    }

    fn generate_robot_name() -> String {

        let uppercase_letters_table: HashMap<usize, char> =
            vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
                .into_iter()
                .enumerate()
                .collect();

        let mut rng = rand::thread_rng();
        let alpha_range = Range::new(0, uppercase_letters_table.len());
        let numeric_range = Range::new(100, 1000);

        format!("{}{}{}",
                uppercase_letters_table[&alpha_range.ind_sample(&mut rng)],
                uppercase_letters_table[&alpha_range.ind_sample(&mut rng)],
                numeric_range.ind_sample(&mut rng))
    }
}
