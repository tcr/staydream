use std::process::Command;
use std::process::Stdio;
use std::io::prelude::*;

const RAIN: &'static [u8] = include_bytes!("../data/rain.mp3");

fn main() {
    loop {
        println!("playing rain...");
        let mut play = Command::new("play")
            .arg("-t").arg("mp3")
            .arg("-")
            .arg("trim").arg("20").arg("140")
            .arg("repeat")
            .arg("-")
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        if let Some(ref mut stdin) = play.stdin {
            stdin.write(RAIN);
        }

        let output = play
            .wait_with_output()
            .expect("failed to wait on child");
    }
}
