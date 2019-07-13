use rand::Rng;
use rand::seq::SliceRandom;

use crate::utility::application_error::ApplicationError;
use crate::utility::read_file::read_file;
use crate::utility::date::Date;

use super::Person;

pub struct PersonGenerator {
    next_id: u32,
    curr_date: Date,
    first_names_male: Vec<String>,
    first_names_female: Vec<String>,
    last_names: Vec<String>
}

impl PersonGenerator {
    pub fn new() -> Result<Self, ApplicationError> {
        let mut pg = Self {
            next_id: 0,
            curr_date: Date::default(),
            first_names_male: Vec::new(),
            first_names_female: Vec::new(),
            last_names: Vec::new()
        };

        pg.load_names()?;

        Ok(pg)
    }

    pub fn set_date(&mut self, new_date: Date) {
        self.curr_date = new_date;
    }

    pub fn generate_random_person<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Person {
        let mut p = Person::new(self.next_id);
        self.next_id += 1;
        
        if rng.gen_bool(0.5) {
            p.set_male();
            p.set_first_name(self.get_random_male_first_name(rng));
        } else {
            p.set_female();
            p.set_first_name(self.get_random_female_first_name(rng));
        }
        p.set_birthday(self.curr_date);
        p.set_last_name(&self.get_random_last_name(rng));

        p
    }

    fn load_names(&mut self) -> Result<(), ApplicationError> {
        info!("Loading names");
        let first_names_male = read_file("resources/names_first_male.txt")?;
        for name in first_names_male.split("\n") {
            self.first_names_male.push(String::from(name));
        }
        info!("Loaded {} male first names", self.first_names_male.len());
        
        let first_names_female = read_file("resources/names_first_female.txt")?;
        for name in first_names_female.split("\n") {
            self.first_names_female.push(String::from(name));
        }
        info!("Loaded {} female first names", self.first_names_female.len());

        let last_names = read_file("resources/names_last.txt")?;
        for name in last_names.split("\n") {
            self.last_names.push(String::from(name));
        }
        info!("Loaded {} last names", self.last_names.len());
        info!("Finished loading of names");
        Ok(())
    }

    fn get_random_male_first_name<R: Rng + ?Sized>(&self, rng: &mut R) -> &str {
        match self.first_names_male.choose(rng) {
            Some(name) => name,
            None => "Nameless"
        }
    }

    fn get_random_female_first_name<R: Rng + ?Sized>(&self, rng: &mut R) -> &str {
        match self.first_names_female.choose(rng) {
            Some(name) => name,
            None => "Nameless"
        }
    }

    fn get_random_last_name<R: Rng + ?Sized>(&mut self, rng: &mut R) -> String {
        let name = self.last_names.choose(rng);
        match name {
            Some(n) => n.clone(),
            None => String::from("McNamelessFace")
        }  
    }


}
