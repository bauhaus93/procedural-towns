use std::fmt;
use rand::Rng;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Gender {
    MALE,
    FEMALE
}

pub fn get_random_gender<R: Rng + ?Sized>(rng: &mut R) -> Gender {
    let g = rng.gen_range(0, 2);
    match g {
        0 => Gender::MALE,
        1 => Gender::FEMALE,
        _ => unreachable!()
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::MALE => write!(f, "male"),
            Gender::FEMALE => write!(f, "female")
        }
    }
}
