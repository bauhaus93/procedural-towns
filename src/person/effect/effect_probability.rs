use rand::{ Rng, SeedableRng };
use rand::rngs::StdRng;

use std::collections::BTreeMap;

pub struct EffectProbability {
    rng: StdRng,
    steps_per_year: u32,
    probabilities: BTreeMap<String, f64>,
}

impl EffectProbability {
    pub fn new<R: Rng + ?Sized>(steps_per_year: u32, rng: &mut R) -> Self {
        let mut ep = Self {
            rng: StdRng::from_rng(rng).unwrap(),
            steps_per_year: steps_per_year,
            probabilities: BTreeMap::new()
        };

        ep.fill_default();

        ep
    }

    fn fill_default(&mut self) {
        self.set_expected_occurences("pregnancy", 2);
    }

    fn set_expected_occurences(&mut self, effect_name: &str, expected_occurences_per_year: u32) {
        self.probabilities.insert(String::from(effect_name), expected_occurences_per_year as f64 / self.steps_per_year as f64);
    }

    fn check_effect(&mut self, effect_name: &str) -> bool {
        match self.probabilities.get(effect_name) {
            Some(prob) => self.rng.gen::<f64>() < *prob,
            None => {
                warn!("Could not find effect '{}' in effect probability list", effect_name);
                false
            }
        }
    }

}
