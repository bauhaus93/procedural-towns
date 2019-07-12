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
            Some(p) => {
                let pid = p.get_id();
                self.extract_person(pid)
            },
            None => None
        }
    }

    pub fn random_marriage(&mut self) -> bool {
        match (self.extract_random_unmarried(Gender::MALE),
               self.extract_random_unmarried(Gender::FEMALE)) {
            (Some(mut hb), Some(mut wf)) => {
                marry(&mut hb, &mut wf);
                self.add_inhabitant(hb);
                self.add_inhabitant(wf);
                true
            },
            (Some(hb), None) => {
                self.add_inhabitant(hb);
                false
            },
            (None, Some(wf)) => {
                self.add_inhabitant(wf);
                false
            },
            (None, None) => false
        }
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
