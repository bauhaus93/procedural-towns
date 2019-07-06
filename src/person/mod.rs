pub mod person;
pub mod person_generator;
pub mod gender;

pub use self::person::Person;
pub use self::person_generator::PersonGenerator;
pub use self::gender::{ Gender, get_random_gender };
