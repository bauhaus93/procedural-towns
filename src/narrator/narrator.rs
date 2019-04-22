
use crate::town::town::Town;
use crate::utility::date::Date;

pub trait Narrator {
    fn narrate(&mut self, town: &mut Town);
    fn get_date(&self) -> Date;
    fn skip_random_years(&mut self, range: (u32, u32));
    fn found_town(&mut self) -> Town;
}