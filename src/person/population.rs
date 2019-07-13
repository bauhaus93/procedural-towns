use std::fmt;
use rand:: { Rng, seq::IteratorRandom };

use crate::utility::{ Date, DAYS_PER_MONTH };
use super::{ Person, AttributeList, PersonGenerator, Attribute };

#[derive(Clone)]
pub struct Population {
    population: Vec<Person>,
    capacity: u32,
    growth_accumulator: f32
}

impl Population {

    pub fn new<R: Rng + ?Sized>(size: u32, capacity: u32, generator: &mut PersonGenerator, rng: &mut R) -> Population {
        let mut population = Population::default();
        population.set_capacity(capacity);
        for _ in 0..size {
            let person = generator.generate_random_person(rng);
            population.add(person);
        }
        population
    }

    pub fn can_grow(&self) -> bool {
        self.growth_accumulator > 1.
    }

    pub fn calculate_growth(&self) -> f32 {
        const COEFF: f32 = 0.05;
        logistic(self.size() as f32, self.capacity as f32, COEFF)
    }

    pub fn apply_growth(&mut self) {
        self.growth_accumulator += self.calculate_growth();
        if self.growth_accumulator.abs() > 10. {
            self.growth_accumulator = 10. * self.growth_accumulator.signum();
        }
    }

    pub fn size(&self) -> u32 {
        self.population.len() as u32
    }

    pub fn add(&mut self, person: Person) {
        self.population.push(person);
    }

    pub fn set_capacity(&mut self, new_cap: u32) {
        self.capacity = new_cap;
    }

    pub fn get_population(&self) -> &[Person] {
        &self.population
    }

    pub fn randomize_birthdays<R: Rng + ?Sized>(&mut self, now: &Date, past_years: (u32, u32), rng: &mut R) {
        self.population.iter_mut()
            .for_each(|p| p.set_birthday(now.random_past_years_range(past_years, rng)));
    }

    pub fn get_random_by_attribute_list<R: Rng + ?Sized>(&self, wanted: &AttributeList, unwanted: &AttributeList, rng: &mut R) -> Option<&Person> {
        match self.get_random_index_by_attribute_list(wanted, unwanted, rng) {
            Some(index) => Some(&self.population[index]),
            None => None
        }
    }
    pub fn pop_random_by_attribute_list<R: Rng + ?Sized>(&mut self, wanted: &AttributeList, unwanted: &AttributeList, rng: &mut R) -> Option<Person> {
        match self.get_random_index_by_attribute_list(wanted, unwanted, rng) {
            Some(index) => Some(self.pop_by_index(index)),
            None => None
        }
    }

    pub fn pop_by_attribute_list(&mut self, wanted: &AttributeList, unwanted: &AttributeList) -> Vec<Person> {
        let mut chosen = Vec::new();
        while let Some(index) = self.get_first_index_by_attribute_list(wanted, unwanted) {
            chosen.push(self.pop_by_index(index));
        }
        chosen
    }

    pub fn get_by_id(&self, id: u32) -> Option<&Person> {
        self.population.iter()
            .find(|p| p.get_id() == id)
    }

    fn pop_by_id(&mut self, id: u32) -> Person {
        let opt_index = self.population.iter()
            .enumerate()
            .find_map(|(i, p)|
                if p.get_id() == id {
                    Some(i)
                } else {
                    None
                });
        match opt_index {
            Some(i) => self.pop_by_index(i),
            None => unreachable!("Id could not be found")
        }
    }                

    fn pop_by_index(&mut self, index: usize) -> Person {
        debug_assert!(index < self.population.len());
        self.population.swap_remove(index)
    }

    pub fn random_marriage<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let wanted_male = AttributeList::builder().set_male().build();
        let wanted_female  = AttributeList::builder().set_female().build();
        let unwanted = AttributeList::builder().set_married().build();

        let opt_male = self.pop_random_by_attribute_list(&wanted_male, &unwanted, rng);
        let opt_female = self.pop_random_by_attribute_list(&wanted_female, &unwanted, rng);
        match (opt_male, opt_female) {
            (Some(mut male), Some(mut female)) => {
                info!("Marrying: {} and {}", male.get_full_name(), female.get_full_name());
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

    pub fn random_pregnancy<R: Rng + ?Sized>(&mut self, today: Date, rng: &mut R) -> bool {
        const MOTHER_MANDATORY_MARRIED_CHANCE: f64 = 0.9;
        const FATHER_EXTRA_MARRIAGE_CHANCE: f64 = 0.2;
        const PREGNANCY_LENGTH: u32 = DAYS_PER_MONTH * 9; 
        let wanted_mother = match rng.gen_bool(MOTHER_MANDATORY_MARRIED_CHANCE) {
            true => AttributeList::builder().set_female().set_married().build(),
            false => AttributeList::builder().set_female().build()
        };
        let unwanted_mother = AttributeList::builder().set_pregnant().build();
        let wanted_father = AttributeList::builder().set_male().build();
        let unwanted_father = match rng.gen_bool(FATHER_EXTRA_MARRIAGE_CHANCE) {
            true => AttributeList::default(),
            false => AttributeList::builder().set_married().build()
        };

        let opt_mother = self.pop_random_by_attribute_list(&wanted_mother, &unwanted_mother, rng);
        match opt_mother {
            Some(mut mother) => {
                let opt_father = match mother.get_spouse() {
                    Some(spouse_id) => self.get_by_id(spouse_id),
                    None => self.get_random_by_attribute_list(&wanted_father, &unwanted_father, rng)
                };
                match opt_father {
                    Some(father) => {
                        let birthday = today + rng.gen_range(PREGNANCY_LENGTH - 10, PREGNANCY_LENGTH + 10);
                        mother.get_attr_mut().set_pregnant(father.get_id(), birthday);
                        info!("New pregnancy: mother: {}, father: {}, birthday: {}", mother.get_full_name(), father.get_full_name(), birthday);
                        self.add(mother);
                        self.growth_accumulator -= 1.;
                        true
                    },
                    None => {
                        self.add(mother);
                        trace!("Wanted to spawn pregnancy, but no father found");
                        true
                    }
                }
            },
            None => {
                trace!("Wanted to spawn pregnancy, but no mother found");
                false
            }
        }
    }

    pub fn handle_births<R: Rng + ?Sized>(&mut self, today: Date, person_generator: &mut PersonGenerator, rng: &mut R) {
        let wanted = AttributeList::builder().set_pregnant().build();
        let unwanted = AttributeList::default();

        for mut mother in self.pop_by_attribute_list(&wanted, &unwanted) {
            match mother.get_attr_mut().pop_pregnancy() {
                Some(Attribute::Pregnant { father_id, birth }) if today >= birth => {
                    let mut child = person_generator.generate_random_person(rng);
                    child.set_last_name(mother.get_last_name());
                    child.set_father(father_id);
                    child.set_mother(mother.get_id());
                    child.set_birthday(birth);
                    info!("New child: {}, mother: {}", child.get_full_name(), mother.get_full_name());
                    self.population.push(child);
                    self.population.push(mother);
                },
                Some(attr @ Attribute::Pregnant { .. } ) => {
                    mother.get_attr_mut().add(attr);
                    self.population.push(mother);
                },
                Some(_unexpected_attr) => unreachable!("Attribute should have been Attribute::Pregnant"),
                None => unreachable!("Target should have been pregnant")
            }
        }
    }

    pub fn handle_deaths<R: Rng + ?Sized>(&mut self, today: Date, rng: &mut R) {
        self.handle_death_by_age(today, rng);
    }

    fn handle_death_by_age<R: Rng + ?Sized>(&mut self, today: Date, rng: &mut R) {
        let mut death_list = Vec::new();
        for person in &self.population {
            let age = person.get_age(&today);
            info!("Death chance for age = {}: {}", age, death_by_age_probability(age));
            let die = rng.gen_bool(death_by_age_probability(age) as f64);
            if die {
                info!("{} ({}) dies of old age", person.get_full_name(), age);
                death_list.push(person.get_id());
            }
        }
        death_list.into_iter().for_each(|id| self.kill_person(id));
    }

    fn kill_person(&mut self, id: u32) {
        let person = self.pop_by_id(id);
        match person.get_spouse() {
            Some(spouse_id) => {
                let mut spouse = self.pop_by_id(spouse_id);
                spouse.get_attr_mut().pop_marriage();
                info!("{} is now a widow/er", spouse.get_full_name());
                self.add(spouse);
            },
            None => {}
        }
    }
    
    fn get_random_index_by_attribute_list<R: Rng + ?Sized>(
        &self,
        wanted: &AttributeList,
        unwanted: &AttributeList,
        rng: &mut R) -> Option<usize> {
        self.population.iter()
            .enumerate()
            .filter_map(|(i, p)|
                if p.satisfies(wanted, unwanted) {
                    Some(i)
                } else {
                    None
                })
            .choose(rng)
    }

    fn get_first_index_by_attribute_list(&self, wanted: &AttributeList, unwanted: &AttributeList) -> Option<usize> {
        self.population.iter()
            .enumerate()
            .find_map(|(i, p)|
                if p.satisfies(wanted, unwanted) {
                    Some(i)
                } else {
                    None
                })
    }
}

impl Default for Population {
    fn default() -> Population {
        Self {
            population: Vec::new(),
            capacity: 50,
            growth_accumulator: 0.
        }
    }
}

impl fmt::Display for Population  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size = {}, capacity = {}, growth = {}, growth acc = {}", self.population.len(), self.capacity, self.calculate_growth(), self.growth_accumulator)
    }
}

fn logistic(curr: f32, cap: f32, coefficent: f32) -> f32 {
    coefficent * curr * (1. - curr / cap)
}

fn death_by_age_probability(age: u32) -> f32 {
    1. - 1. / f32::exp((age as f32 - 26.).abs() * 1e-3)
}
