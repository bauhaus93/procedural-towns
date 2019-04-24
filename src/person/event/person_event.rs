
pub use crate::utility::date::Date;
pub use crate::event::Event;
pub use crate::person::Person;
pub use super::event_type::EventType;

pub struct PersonEvent {
    start: Date,
    target: u32,
    event_type: EventType
}


impl Event<Person> for PersonEvent {
    fn update(&mut self, target: &mut Person, today: Date) {

    }

}