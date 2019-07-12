
use crate::town::town::Town;
use crate::utility::date::Date;

pub trait Narrator {
    fn progress_town(&mut self, town: &Town) -> Town;
    fn increment_date(&mut self);
    fn get_date(&self) -> Date;
    fn skip_random_years(&mut self, range: (u32, u32));
    fn found_town(&mut self) -> Town;
}
