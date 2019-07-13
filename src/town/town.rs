use std::fmt;
use rand::rngs::SmallRng;
use rand::{ Rng, SeedableRng };

use crate::utility::date::{ Date, DAYS_PER_YEAR };
use crate::person::{ Population, PersonGenerator };

#[derive(Clone)]
pub struct Town {
    rng: SmallRng,
    name: String,
    date: Date,
    population: Population,
}

impl Town {
    pub fn found<R: Rng + ?Sized>(name: &str, date: Date, person_generator: &mut PersonGenerator, rng: &mut R) -> Town {
        let mut local_rng = SmallRng::from_rng(rng).unwrap();

        let initial_pop_size = local_rng.gen_range(10, 20);
        let capacity = local_rng.gen_range(initial_pop_size * 2, initial_pop_size * 5);
        let mut population = Population::new(initial_pop_size, capacity, person_generator, &mut local_rng);
        population.randomize_birthdays(&date, (20, 40), &mut local_rng);
        
        let marriage_count = local_rng.gen_range(2, 4);
        for _ in 0..marriage_count {
            population.random_marriage(&mut local_rng);
        }

        Town {
            rng: local_rng,
            name: name.to_owned(),
            date: date,
            population: population,
        }
    }

    pub fn progress_year(&self, person_generator: &mut PersonGenerator) -> Town {
        info!("########################");
        info!("Progressing '{}'", self.name);
        let mut next_town = self.clone();
        next_town.forward_date_one_year();
        next_town.handle_population(person_generator);
        info!("Date: {}, population: {}",
            next_town.get_date(),
            next_town.get_population());
        info!("########################");
        next_town
    }

    fn forward_date_one_year(&mut self) {
        self.date += DAYS_PER_YEAR;
    }

   fn handle_population(&mut self, person_generator: &mut PersonGenerator) {
        self.population.handle_births(self.date, person_generator, &mut self.rng);
        self.population.handle_deaths(self.date, &mut self.rng);
        self.population.apply_growth();
        while self.population.can_grow() {
            if !self.random_pregnancy() {
                break;
            }
        }
    }

    fn random_pregnancy(&mut self) -> bool {
        let start = self.date + self.rng.gen_range(0, DAYS_PER_YEAR);
        if self.population.random_pregnancy(start, &mut self.rng) {
            true
        } else {
            false
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_date(&self) -> &Date {
        &self.date
    }

    pub fn print_full(&self) {
        info!("################");
        info!("{}", self);

        self.population.get_population().iter()
            .for_each(|p| info!("{} ({})", p.get_full_name(), p.get_age(&self.date)));
        info!("################");
    }

    fn get_population(&self) -> &Population {
        &self.population
    }
}



impl fmt::Display for Town {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Town {} in {}, inhabitants: {}", self.name, self.date, self.population.size())
    }
}
