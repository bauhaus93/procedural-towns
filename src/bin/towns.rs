#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate procedural_towns;

use std::io::Write;
use env_logger::{ Builder, fmt::Formatter };
use log::Record;
use rand;
use rand::{ Rng, FromEntropy, SeedableRng };
use rand::rngs::StdRng;

use procedural_towns::person::person::Person;
use procedural_towns::person::person_generator::PersonGenerator;

pub fn main() {
    init_custom_logger();
    let mut rng = StdRng::from_entropy();
    info!("Started");
    let mut pg = PersonGenerator::new(&mut rng).unwrap();
    for _ in 0..100 {
        let p = pg.generate_random_person();
        info!("{}", p);
    }

    info!("Finished")
}

fn init_custom_logger() {
    let format = |buf: &mut Formatter, record: &Record| {
        let time = chrono::Local::now();
        writeln!(buf, "[{} {:-5}] {}", time.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args()) 
    };
    Builder::from_default_env()
        .format(format)
        .init();
}