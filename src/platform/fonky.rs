use super::Platform;

use std::fs::metadata;
use std::process::Command;

pub struct Fonky {}

impl Platform for Fonky {
    fn probe(&self) -> bool {
        metadata("./fonky.txt")
            .map(|data| data.is_file())
            .unwrap_or(false)
    }

    fn build(&self) -> bool {
        println!("building a Fonky project");

        let output = Command::new("cp").arg("fonky.txt").arg("fonky.out").output().expect(
            "Fonky build failed",
        );

        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        output.status.success()

    }

    fn run(&self) -> bool {
        println!("running a Fonky project");

        let output = Command::new("cat").arg("fonky.out").output().expect(
            "Fonky run failed",
        );

        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));

        output.status.success()
    }
}