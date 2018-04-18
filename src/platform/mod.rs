mod rust;
use self::rust::Rust;

mod fonky;
use self::fonky::Fonky;

pub trait Platform {
    fn probe(&self) -> bool;

    fn build(&self) -> bool;

    fn run(&self) -> bool;
}

pub fn probe() -> Option<Box<Platform>> {
    let rust = Rust {};
    let fonky = Fonky {};
    if fonky.probe() {
        Some(Box::new(fonky))
    } else if rust.probe() {
        Some(Box::new(rust))
    } else {
        None
    }
}
