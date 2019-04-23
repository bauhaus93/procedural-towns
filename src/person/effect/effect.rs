
use crate::utility::date::Date;
use crate::person::person::Person;
use super::effect_type::EffectType;

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
            effect_type: EffectType::Pregnancy(father.get_id())
        }
    }
    pub fn set_end(&mut self, date: Date) {
        self.end = Some(date);
    }
}

