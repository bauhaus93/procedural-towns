use rand::{ Rng, FromEntropy };
use rand::rngs::StdRng;

use crate::utility::date::Date;
use crate::utility::application_error::ApplicationError;
use crate::town::town::Town;
use crate::person::person_generator::PersonGenerator;

pub struct World {
    rng: StdRng,
    creator: String,
    creation_date: Date,
    curr_date: Date,
    person_generator: PersonGenerator,
    towns: Vec<Town>
}

impl World {
    pub fn create() -> Result<Self, ApplicationError> {
        let mut rng = StdRng::from_entropy();

        let creation_date = Date::random(&mut rng);
        let person_generator = PersonGenerator::new(&mut rng)?;

        let mut world = Self {
            rng: rng,
            creator: String::from("C'thulu"),
            creation_date: creation_date,
            curr_date: creation_date,
            person_generator: person_generator,
            towns: Vec::new()
        };

        let forwarded_date = creation_date.random_future_years_range(100, 1000, &mut world.rng);
        world.set_date(forwarded_date);

        Ok(world)
    }

    pub fn set_date(&mut self, new_date: Date) {
        self.curr_date = new_date;
        self.person_generator.set_date(new_date);
    }

    pub fn add_town(&mut self) {
        let mut town = Town::found(&mut self.rng, "Townshire", self.curr_date);
        info!("New town '{}', founded in {}", town.get_name(), town.get_founding_date());

        let couple_count = self.rng.gen_range(1, 5);
        for _ in 0..couple_count {
            let mut couple = self.person_generator.generate_couple();
            couple.0.set_birthday(couple.0.get_birthday().random_past_years_range(20, 40, &mut self.rng));
            couple.1.set_birthday(couple.1.get_birthday().random_past_years_range(20, 40, &mut self.rng));

            info!("Added couple: {} {} ({}y) and {} {} ({}y)",
                couple.0.get_first_name(),
                couple.0.get_last_name(),
                couple.0.get_age(&self.curr_date),
                couple.1.get_first_name(),
                couple.1.get_last_name(),
                couple.1.get_age(&self.curr_date)
            );

            town.add_inhabitant(couple.0);
            town.add_inhabitant(couple.1);
        }
        let single_count = self.rng.gen_range(4, 8);
        for _ in 0..single_count {
            let mut person = self.person_generator.generate_random_person();
            person.set_birthday(person.get_birthday().random_past_years_range(20, 30, &mut self.rng));
            info!("Added person: {} {} ({}y)",
                person.get_first_name(),
                person.get_last_name(),
                person.get_age(&self.curr_date)
            );
            town.add_inhabitant(person);
        }
        info!("Finished founding of {} ({} inhabitants)", town.get_name(), town.get_size());
        self.towns.push(town);
    }
}