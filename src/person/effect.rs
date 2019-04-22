
use crate::utility::date::Date;
use super::person::Person;

pub struct Effect {
    start: Date,
    end: Option<Date>,
    effect_type: EffectType
}

impl Effect {
    pub fn new_pregnancy(date: Date, father: &Person) -> Self {
        Self {
            start: date,
            end: None,
            effect_type: EffectType::Pregnant(father.get_id())
        }
    }
    pub fn set_end(&mut self, date: Date) {
        self.end = Some(date);
    }
}

pub enum EffectType {
    Pregnant(u32)
}