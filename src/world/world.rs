use rand::FromEntropy;
use rand::rngs::StdRng;

use crate::utility::date::Date;
use crate::utility::application_error::ApplicationError;
use crate::town::town::Town;
use crate::person::PersonGenerator;

#[allow(unused)]
pub struct World {
    rng: StdRng,
    person_generator: PersonGenerator,
    towns: Vec<Town>
}

impl World {
    pub fn create(town_count: u32) -> Result<Self, ApplicationError> {
        let mut rng = StdRng::from_entropy();
        let start_date = Date::random(500, 4000, &mut rng);
        let mut person_generator = PersonGenerator::new()?;

        let mut towns = Vec::new();
        for _ in 0..town_count {
            towns.push(Town::found("Townshire", start_date, &mut person_generator, &mut rng));
        }

        let world = Self {
            rng: rng,
            person_generator: person_generator,
            towns: towns
        };

        Ok(world)
    }

    pub fn progress(&mut self) {
        info!("Progressing world...");
        let mut next_gen = Vec::new();
        for town in &self.towns {
            next_gen.push(town.progress_year(&mut self.person_generator));
        }
        self.towns = next_gen;
    }
}
