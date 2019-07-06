use rand::{ Rng, SeedableRng };
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;

use crate::utility::date::Date;
use crate::person::{ Person, Gender };

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
            name: name.to_owned(),
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

    fn extract_person(&mut self, id: u32) -> Option<Person> {
        let index = match self.inhabitants.iter()
            .enumerate()
            .find(|(i, p)| p.get_id() == id) {
            Some((index,_person)) => index,
            None => return None
        };
        Some(self.inhabitants.remove(index))
    }

    fn extract_random_unmarried(&mut self, gender: Gender) -> Option<Person> {
        match self.inhabitants.iter()
            .filter(|p| !p.is_married() && p.get_gender() == gender)
            .choose(&mut self.rng) {
            Some(p) => self.extract_person(p.get_id()),
            None => unreachable!("No unmarrried person with specified gender in town")
        }
    }

    fn get_person_mut(&self, id: u32) -> Option<&Person> {
        self.inhabitants.iter()
            .find(|p| p.get_id() == id)
    }

    pub fn random_marriage(&mut self) -> bool {
        let mut husband = match self.extract_random_unmarried(Gender::MALE) {
            Some(hb) => hb,
            None => return false
        };
        let mut wife = match self.extract_random_unmarried(Gender::FEMALE) {
            Some(wf) => wf,
            None => return false
        };
        marry(&mut husband, &mut wife);
        self.add_inhabitant(husband);
        self.add_inhabitant(wife);
        true
    }
}


fn marry(husband: &mut Person, wife: &mut Person) {
    info!("{} and {} are marrying",
        husband.get_full_name(),
        wife.get_full_name());
    husband.set_spouse(wife);
    wife.set_spouse(husband);
    wife.set_last_name(husband.get_last_name());
}
