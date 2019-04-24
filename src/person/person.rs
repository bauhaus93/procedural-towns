use std::fmt;

use crate::utility::date::Date;
use super::Gender;

pub struct Person {
    id: u32,
    birthday: Date,
    first_name: String,
    last_name: String,
    gender: Gender,
    father: Option<u32>,
    mother: Option<u32>,
    spouse: Option<u32>,
}

impl Person {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            birthday: Date::default(),
            first_name: String::from("Unknown"),
            last_name: String::from("McUnknownFace"),
            gender: Gender::FEMALE,
            father: None,
            mother: None,
            spouse: None,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }
    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }
    pub fn get_birthday(&self) -> Date {
        self.birthday
    }

    pub fn get_age(&self, curr_date: &Date) -> u32 {
        let had_birthday =
            curr_date.get_month() > self.birthday.get_month() ||
            (curr_date.get_month() == self.birthday.get_month() &&
             curr_date.get_day() >= self.birthday.get_day());
        let age = curr_date.get_year() - self.birthday.get_year() - 1;
        if had_birthday {
            age + 1
        } else {
            age
        }
    }

    pub fn set_birthday(&mut self, birthday: Date) {
        self.birthday = birthday;
    }
    pub fn set_first_name(&mut self, first_name: &str) {
        self.first_name = String::from(first_name);
    }
    pub fn set_last_name(&mut self, last_name: &str) {
        self.last_name = String::from(last_name);
    }
    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }
    pub fn set_father(&mut self, father: &Person) {
        self.father = Some(father.get_id())
    }
    pub fn set_mother(&mut self, mother: &Person) {
        self.mother = Some(mother.get_id());
    }
    pub fn set_spouse(&mut self, spouse: &Person) {
        self.spouse = Some(spouse.get_id());
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id = {}, name = {} {}, gender = {}, birthday = {}", self.id, self.first_name, self.last_name, self.gender, self.birthday)
    }
}