use std::fmt;

use super::gender::Gender;

pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    gender: Gender
}

impl Person {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            first_name: String::from("Nameless"),
            last_name: String::from("McNamelessFace"),
            gender: Gender::FEMALE
        }
    }

    pub fn set_first_name(&mut self, first_name: &str) {
        self.first_name = String::from(first_name);
    }
    pub fn set_last_name(&mut self, last_name: &str) {
        self.last_name = String::from(last_name);
    }
    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id = {}, name = {} {}, gender = {}", self.id, self.first_name, self.last_name, self.gender)
    }
}