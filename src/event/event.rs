use crate::utility::date::Date;

pub trait Event<T> {
    fn update(&mut self, target: &mut T, today: Date);
}