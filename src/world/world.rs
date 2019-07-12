use rand::FromEntropy;
use rand::rngs::StdRng;

use crate::utility::date::Date;
use crate::utility::application_error::ApplicationError;
use crate::town::town::Town;
use crate::narrator::narrator::Narrator;
use crate::narrator::std_narrator::StdNarrator;

#[allow(unused)]
pub struct World {
    rng: StdRng,
    narrator: Box<Narrator>,
    creation_date: Date,
    towns: Vec<Town>
}

impl World {
    pub fn create() -> Result<Self, ApplicationError> {
        let mut rng = StdRng::from_entropy();

        let mut std_narrator = StdNarrator::new(&mut rng)?;
        let creation_date = std_narrator.get_date();
        std_narrator.skip_random_years((100, 500));

        let world = Self {
            rng: rng,
            creation_date: creation_date,
            narrator: Box::new(std_narrator),
            towns: Vec::new()
        };

        Ok(world)
    }
    pub fn add_town(&mut self) {
        let new_town = self.narrator.found_town();
        self.towns.push(new_town);
    }
}
