use std::iter;
use rand::{ Rng, SeedableRng };
use rand::rngs::{ SmallRng, StdRng };

use crate::utility::date::{ Date, DAYS_PER_YEAR };
use crate::utility::application_error::ApplicationError;
use crate::town::town::Town;
use crate::person::PersonGenerator;
use super::narrator::Narrator;

pub struct StdNarrator {
    rng: StdRng,
    curr_date: Date,
    person_generator: PersonGenerator,
    day_increment: u32
}

impl StdNarrator {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Result<Self, ApplicationError> {

        let mut local_rng = StdRng::from_rng(rng).unwrap();
        let day_increment = 7;
        let curr_date = Date::random(0, 5000, &mut local_rng);
        let person_generator = PersonGenerator::new(&mut local_rng)?;
 
        let narrator = Self {
            rng: local_rng,
            curr_date: curr_date,
            person_generator: person_generator,
            day_increment: day_increment
        };

        Ok(narrator)
    }

    pub fn set_date(&mut self, new_date: Date) {
        self.curr_date = new_date;
        self.person_generator.set_date(new_date);
    }
}

impl Narrator for StdNarrator {
    fn narrate(&mut self, town: &mut Town) {

    }

    fn increment_date(&mut self) {
        self.set_date(self.get_date() + self.day_increment);
    }

    fn skip_random_years(&mut self, range: (u32, u32)) {
        let new_date = self.curr_date.random_future_years_range(range, &mut self.rng);
        self.set_date(new_date);
    }

    fn get_date(&self) -> Date {
        self.curr_date
    }

    fn found_town(&mut self) -> Town {
        let mut founding_rng = SmallRng::from_rng(&mut self.rng).unwrap();
        let mut town = Town::found(&mut founding_rng, "Townshire", self.curr_date);
        info!("New town '{}', founded in {}", town.get_name(), town.get_founding_date());

        let person_count = self.rng.gen_range(10, 20);
        for _ in 0..person_count {
            let mut person = self.person_generator.generate_random_person();
            person.set_birthday(self.curr_date.random_past_years_range((20, 40), &mut founding_rng));
            info!("Added person: {} {} ({}y)",
                person.get_first_name(),
                person.get_last_name(),
                person.get_age(&self.curr_date)
            );
            town.add_inhabitant(person);
        }
        let marriage_count = founding_rng.gen_range(2, 4);
        for _ in 0..marriage_count {
            town.random_marriage();
        }
        info!("Finished founding of {} ({} inhabitants)", town.get_name(), town.get_size());
        town
    }
}
