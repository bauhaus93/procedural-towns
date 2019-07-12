use rand:: { Rng, seq::IteratorRandom };

use crate::utility::Date;
use super::{ Person, AttributeList, PersonGenerator };

#[derive(Clone)]
pub struct People {
    people: Vec<Person>
}

impl People {

    pub fn new<R: Rng + ?Sized>(size: u32, generator: &mut PersonGenerator, rng: &mut R) -> People {
        let mut people = People::default();
        for _ in 0..size {
            let person = generator.generate_random_person(rng);
            people.add(person);
        }
        people
    }

    pub fn size(&self) -> u32 {
        self.people.len() as u32
    }

    pub fn add(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn get_people(&self) -> &[Person] {
        &self.people
    }

    pub fn randomize_birthdays<R: Rng + ?Sized>(&mut self, now: &Date, past_years: (u32, u32), rng: &mut R) {
        self.people.iter_mut()
            .for_each(|p| p.set_birthday(now.random_past_years_range(past_years, rng)));
    }

    pub fn pop_random_by_attribute_list<R: Rng + ?Sized>(&mut self, attributes: &AttributeList, rng: &mut R) -> Option<Person> {
        let picked_index = self.people.iter()
            .enumerate()
            .filter_map(|(i, p)|
                if p.satisfies(attributes) {
                    Some(i)
                } else {
                    None
                })
            .choose(rng);
        match picked_index {
            Some(index) => Some(self.people.swap_remove(index)),
            None => None
        }
    }

    pub fn random_marriage<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut male_unmarried = AttributeList::default();
        male_unmarried.set_single().set_male();
        let mut female_unmarried = AttributeList::default();
        female_unmarried.set_single().set_female();

        let opt_male = self.pop_random_by_attribute_list(&male_unmarried, rng);
        let opt_female = self.pop_random_by_attribute_list(&female_unmarried, rng);
        match (opt_male, opt_female) {
            (Some(mut male), Some(mut female)) => {
                info!("Marrying");
                male.get_attr_mut().set_married(female.get_id());
                female.get_attr_mut().set_married(male.get_id());
                female.set_last_name(male.get_last_name());
                self.add(male);
                self.add(female);
            },
            (Some(male), None) => {
                self.add(male);
            },
            (None, Some(female)) => {
                self.add(female);
            },
            (None, None) => {}
        }
    }
}

impl Default for People {
    fn default() -> People {
        Self {
            people: Vec::new()
        }
    }
}
