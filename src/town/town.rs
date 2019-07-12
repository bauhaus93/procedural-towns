use std::fmt;

use crate::utility::date::Date;
use crate::person::{ Person, People };

#[derive(Clone)]
pub struct Town {
    name: String,
    date: Date,
    people: People
}

impl Town {
    pub fn new(name: &str, date: Date, people: People) -> Town {
        Town {
            name: name.to_owned(),
            date: date,
            people: people
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_date(&self) -> &Date {
        &self.date
    }

    pub fn set_people(&mut self, people: People) {
        self.people = people;
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.add(person);
    }

    pub fn get_size(&self) -> u32 {
        self.people.size()
    }

    pub fn print_full(&self) {
        info!("################");
        info!("{}", self);

        self.people.get_people().iter()
            .for_each(|p| info!("{} ({})", p.get_full_name(), p.get_age(&self.date)));
        info!("################");
    }
}



impl fmt::Display for Town {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Town {} in {}, inhabitants: {}", self.name, self.date, self.people.size())
    }
}
