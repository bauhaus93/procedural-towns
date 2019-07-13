use std::fmt;
use rand:: { Rng, seq::IteratorRandom };

use crate::utility::{ Date, DAYS_PER_MONTH, DAYS_PER_YEAR };
use super::{ Person, AttributeList, PersonGenerator, Attribute };

#[derive(Clone)]
pub struct Population {
    population: Vec<Person>,
    capacity: u32,
    growth_accumulator: f32
}

impl Population {

    pub fn new<R: Rng + ?Sized>(size: u32, capacity: u32, today: Date, generator: &mut PersonGenerator, rng: &mut R) -> Population {
        let mut population = Population::default();
        population.set_capacity(capacity);
        for _ in 0..size {
            let mut person = generator.generate_random_person(rng);
            person.set_birthday(today.random_past_years_range((5, 40), rng));
            population.add(person);
        }
        population.handle_fertility(today);

        while population.get_marriage_ratio() < 0.66 {
            if !population.random_marriage(rng) {
                break;
            }
        }
        population
    }

    pub fn can_grow(&self) -> bool {
        self.growth_accumulator > 1.
    }

    pub fn calculate_growth(&self) -> f32 {
        const COEFF: f32 = 0.1;
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

    pub fn get_female_ratio(&self) -> f32 {
        self.population.iter()
            .fold(0, |acc, p|
                if p.get_attr().is_female() {
                    acc + 1
                } else {
                    acc
                }) as f32 / self.population.len() as f32
    }

    pub fn get_fertility_ratio(&self) -> f32 {
        self.population.iter()
            .fold(0, |acc, p|
                if p.get_attr().is_fertile() {
                    acc + 1
                } else {
                    acc
                }) as f32 / self.population.len() as f32
    }

    pub fn get_marriage_ratio(&self) -> f32 {
        self.population.iter()
            .fold(0, |acc, p|
                if p.get_attr().is_married() {
                    acc + 1
                } else {
                    acc
                }) as f32 / self.population.len() as f32
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

    pub fn update<R: Rng + ?Sized>(&mut self, today: Date, person_generator: &mut PersonGenerator, rng: &mut R) {
        self.handle_births(today, person_generator, rng);
        self.handle_deaths(today, rng);
        self.handle_growth(today, rng);
        self.handle_fertility(today);
        self.handle_marriages(rng);
    }

    fn handle_growth<R: Rng + ?Sized>(&mut self, today: Date, rng: &mut R) {
        self.apply_growth();
        while self.can_grow() {
            let preg_start = today + rng.gen_range(0, DAYS_PER_YEAR);
            if !self.random_pregnancy(preg_start, rng) {
                break;
            }
        }
    }

    fn handle_marriages<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut fails = 0;
        while self.get_marriage_ratio() < 0.66 && fails < 3 {
            if !self.random_marriage(rng) {
                fails +=1;
            }
        }
    }

    pub fn random_marriage<R: Rng + ?Sized>(&mut self, rng: &mut R) -> bool {
        let wanted_male = AttributeList::builder().set_male().set_fertile().build();
        let wanted_female  = AttributeList::builder().set_female().set_fertile().build();
        let unwanted = AttributeList::builder().set_married().build();

        match self.pop_random_by_attribute_list(&wanted_male, &unwanted, rng) {
            Some(mut groom) => {
                match self.pop_random_by_attribute_list(&wanted_female, &unwanted, rng) {
                    Some(mut bride) => {
                        info!("Marrying: {} and {}", groom.get_full_name(), bride.get_full_name());
                        groom.get_attr_mut().set_married(bride.get_id());
                        bride.get_attr_mut().set_married(groom.get_id());
                        bride.set_last_name(groom.get_last_name());
                        self.add(groom);
                        self.add(bride);
                        true
                    },
                    None => {
                        self.add(groom);
                        false
                    }
                }
            },
            None => false
        }
    }

    pub fn random_pregnancy<R: Rng + ?Sized>(&mut self, today: Date, rng: &mut R) -> bool {
        const MOTHER_MANDATORY_MARRIED_CHANCE: f64 = 0.6;
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
                        let count = if rng.gen_bool(0.05) {
                            rng.gen_range(2, 5)
                        } else {
                            1
                        };
                        mother.get_attr_mut().set_pregnant(father.get_id(), birthday, count);
                        info!("New pregnancy: mother: {}, father: {}, birthday: {}, count: {}", mother.get_full_name(), father.get_full_name(), birthday, count);
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
                Some(Attribute::Pregnant { father_id, birth, count }) if today >= birth => {
                    for _ in 0..count {
                        let mut child = person_generator.generate_random_person(rng);
                        child.set_last_name(mother.get_last_name());
                        child.set_father(father_id);
                        child.set_mother(mother.get_id());
                        child.set_birthday(birth);
                        info!("New child: {}, mother: {}", child.get_full_name(), mother.get_full_name());
                        self.population.push(child);
                    }
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
            let die = rng.gen_bool(death_by_age_probability(age) as f64);
            if die {
                info!("{} ({}) dies of age", person.get_full_name(), age);
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

    fn handle_fertility(&mut self, today: Date) {
        self.handle_fertility_gain(today);
        self.handle_fertility_lose(today);
    }

    fn handle_fertility_gain(&mut self, today: Date) {
        let wanted = AttributeList::default();
        let unwanted = AttributeList::builder().set_fertile().build();

        for mut person in self.pop_by_attribute_list(&wanted, &unwanted).into_iter() {
            let age = person.get_age(&today);
            if age >= 14 && age < 40 {
                trace!("{} is now fertile", person.get_full_name());
                person.get_attr_mut().set_fertile();
            }
            self.add(person);
        }
    }
    
    fn handle_fertility_lose(&mut self, today: Date) {
        let wanted = AttributeList::builder().set_fertile().build();
        let unwanted = AttributeList::default();

        for mut person in self.pop_by_attribute_list(&wanted, &unwanted).into_iter() {
            if person.get_age(&today) >= 40 {
                trace!("{} lost fertility", person.get_full_name());
                person.get_attr_mut().clear_fertile();
            }
            self.add(person);
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
        write!(f, "size = {}, capacity = {}, female ratio = {:.2}, fertility ratio = {:.2}, marriage ratio = {:.2}, growth = {}, growth acc = {}",
            self.population.len(),
            self.capacity,
            self.get_female_ratio(),
            self.get_fertility_ratio(),
            self.get_marriage_ratio(),
            self.calculate_growth(),
            self.growth_accumulator)
    }
}

fn logistic(curr: f32, cap: f32, coefficent: f32) -> f32 {
    coefficent * curr * (1. - curr / cap)
}

fn death_by_age_probability(age: u32) -> f32 {
    1. - 1. / f32::exp((age as f32 - 26.).powf(2.) * 1e-4)
}
