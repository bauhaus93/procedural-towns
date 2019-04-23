use rand::{ Rng, SeedableRng };
use rand::rngs::StdRng;

use crate::utility::date::Date;
use crate::person::person::Person;

pub struct Town {
    rng: StdRng,
    name: String,
    foundation: Date,
    inhabitants: Vec<Person>
}

impl Town {
    pub fn found<R: Rng + ?Sized>(rng: &mut R, name: &str, date: Date) -> Self {
        Self {
            rng: StdRng::from_rng(rng).unwrap(),
            name: String::from(name),
            foundation: date,
            inhabitants: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_founding_date(&self) -> &Date {
        &self.foundation
    }

    pub fn add_inhabitant(&mut self, person: Person) {
        self.inhabitants.push(person);
    }

    pub fn get_size(&self) -> usize {
        self.inhabitants.len()
    }
}