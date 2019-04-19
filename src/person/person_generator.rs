use rand;
use rand::{ Rng, SeedableRng };
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::utility::application_error::ApplicationError;
use crate::utility::read_file::read_file;
use crate::utility::date::Date;

use super::person::Person;
use super::gender::{ Gender, get_random_gender };

pub struct PersonGenerator {
    rng: StdRng,
    next_id: u32,
    curr_date: Date,
    first_names_male: Vec<String>,
    first_names_female: Vec<String>,
    last_names: Vec<String>
}

impl PersonGenerator {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Result<Self, ApplicationError> {
        let mut pg = Self {
            rng: StdRng::from_rng(rng).unwrap(),
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

    pub fn generate_random_person(&mut self) -> Person {
        let mut p = Person::new(self.next_id);
        self.next_id += 1;

        let gender = get_random_gender(&mut self.rng);
        p.set_gender(gender);
        p.set_birthday(self.curr_date);
        p.set_first_name(&self.get_random_first_name(gender));
        p.set_last_name(&self.get_random_last_name());

        p
    }

    fn generate_random_with_gender(&mut self, gender: Gender) -> Person {
        let mut p = self.generate_random_person();
        p.set_gender(gender);
        p.set_first_name(&self.get_random_first_name(gender));
        p
    }

    pub fn generate_couple(&mut self) -> (Person, Person) {
        let mut husband = self.generate_random_with_gender(Gender::MALE);
        let mut wife = self.generate_random_with_gender(Gender::FEMALE);

        husband.set_spouse(&wife);
        wife.set_spouse(&husband);
        wife.set_last_name(husband.get_last_name());

        (husband, wife)
    }

    pub fn generate_child(&mut self, father: &Person, mother: &Person) -> Person {
        let mut p = self.generate_random_person();
        
        p.set_last_name(mother.get_last_name());
        p.set_father(father);
        p.set_mother(mother);
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

    fn get_random_first_name(&mut self, gender: Gender) -> String {
        let name = match gender {
            Gender::MALE => self.first_names_male.choose(&mut self.rng),
            Gender::FEMALE => self.first_names_female.choose(&mut self.rng)
        };
        match name {
            Some(n) => n.clone(),
            None => String::from("Nameless")
        }
    }

    fn get_random_last_name(&mut self) -> String {
        let name = self.last_names.choose(&mut self.rng);
        match name {
            Some(n) => n.clone(),
            None => String::from("McNamelessFace")
        }  
    }


}