use crate::utility::date::Date;

use super::Event;

pub trait EventSpawner<T> {
    fn spawn_events(&mut self, target: &T, today: Date) -> Vec<Box<Event<T>>>;
}