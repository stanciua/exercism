#[macro_use]
extern crate lazy_static;
extern crate rand;


use std::collections::{HashMap, HashSet};
use rand::distributions::{IndependentSample, Range};
use std::sync::Mutex;


lazy_static! {
    pub static ref ROBOTSREGISTRY: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Default)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: Robot::generate_robot_name_with_registry_update() }
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn reset_name(&mut self) {
        Robot::remove_robot_name_from_registry(&*self.name);
        self.name.clear();
        self.name = Robot::generate_robot_name_with_registry_update();
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

    fn generate_robot_name_with_registry_update() -> String {
        let mut robot_name = Robot::generate_robot_name();
        // Make sure the registry doesn't already have the generated robot name
        while ROBOTSREGISTRY.lock().unwrap().contains(&robot_name) {
            robot_name = Robot::generate_robot_name();
        }

        // add the newly generated name to robot registry
        ROBOTSREGISTRY.lock().unwrap().insert(robot_name.clone());

        robot_name
    }

    fn remove_robot_name_from_registry(name: &str) {
        ROBOTSREGISTRY.lock().unwrap().remove(name);
    }
}
