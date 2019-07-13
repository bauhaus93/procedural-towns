use std::fmt;

use crate::utility::date::Date;
use super::AttributeList;

#[derive(Clone)]
pub struct Person {
    id: u32,
    birthday: Date,
    first_name: String,
    last_name: String,
    father: Option<u32>,
    mother: Option<u32>,
    attributes: AttributeList
}

impl Person {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            birthday: Date::default(),
            first_name: String::from("Unknown"),
            last_name: String::from("McUnknownFace"),
            father: None,
            mother: None,
            attributes: AttributeList::default()
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
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn get_birthday(&self) -> Date {
        self.birthday
    }

    pub fn get_spouse(&self) -> Option<u32> {
        self.attributes.get_spouse()
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
    pub fn set_father(&mut self, father_id: u32) {
        self.father = Some(father_id);
    }
    pub fn set_mother(&mut self, mother_id: u32) {
        self.mother = Some(mother_id);
    }

    pub fn set_male(&mut self) {
        self.attributes.set_male();
    }
    pub fn set_female(&mut self) {
        self.attributes.set_female();
    }

    pub fn satisfies(&self, wanted_attributes: &AttributeList, unwanted_attributes: &AttributeList) -> bool {
        self.attributes.satisfies(wanted_attributes, unwanted_attributes)
    }

    pub fn get_attr(&self) -> &AttributeList {
        &self.attributes
    }
    pub fn get_attr_mut(&mut self) -> &mut AttributeList {
        &mut self.attributes
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id = {}, name = {} {}, birthday = {}", self.id, self.first_name, self.last_name, self.birthday)
    }
}
