#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate procedural_towns;

use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use env_logger::{ Builder, fmt::Formatter };
use log::Record;

use procedural_towns::world::world::World;

pub fn main() {
    init_custom_logger();

    match World::create(1) {
        Ok(mut world) => {
            loop {
                world.progress();
                sleep(Duration::from_millis(500));
            }
        },
        Err(e) => error!("{}", e)
    }
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
