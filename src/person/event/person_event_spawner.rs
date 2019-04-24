
use crate::utility::date::Date;
use crate::event::{ Event, EventSpawner };
use crate::person::Person;
use super::PersonEvent;

pub struct PersonEventSpawner {

}

impl EventSpawner<Person> for PersonEventSpawner {
    fn spawn_events(&mut self, target: &Person, today: Date) -> Vec<Box<Event<Person>>> {
        Vec::new()
    }
}