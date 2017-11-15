extern crate sysbar;
extern crate tempdir;

use std::process::{Command, Child};
use std::process::Stdio;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::path::Path;
use tempdir::TempDir;
use std::fs::File;

const RAIN: &'static [u8] = include_bytes!("../data/rain.mp3");

fn main() {
    let dir = TempDir::new("staydream").expect("could not create temp dir");
    let file_path = dir.path().join("rain.mp3");
    println!("{:?}", file_path);

    let mut f = File::create(&file_path).unwrap();
    f.write_all(RAIN).unwrap();
    f.sync_all().unwrap();


    let mut bar = sysbar::Sysbar::new("☔️");

    let data: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));

    let data2 = data.clone();
    let fp = file_path.clone();
    bar.add_item(
        "Play 100%",
        Box::new(move || {
            let data2 = data2.clone();
            let fp = fp.clone();
            spawn(move || {
                let mut value = data2.lock().expect("could not unwrap mutex");
                if let Some(mut child) = value.take() {
                    child.kill()
                        .expect("failed to wait on child");
                }
                *value = Some(start_audio(&fp, 100));
            });
        }),
    );

    let data2 = data.clone();
    let fp = file_path.clone();
    bar.add_item(
        "Play 75%",
        Box::new(move || {
            let data2 = data2.clone();
            let fp = fp.clone();
            spawn(move || {
                let mut value = data2.lock().expect("could not unwrap mutex");
                if let Some(mut child) = value.take() {
                    child.kill()
                        .expect("failed to wait on child");
                }
                *value = Some(start_audio(&fp, 75));
            });
        }),
    );

    let data2 = data.clone();
    let fp = file_path.clone();
    bar.add_item(
        "Play 50%",
        Box::new(move || {
            let data2 = data2.clone();
            let fp = fp.clone();
            spawn(move || {
                let mut value = data2.lock().expect("could not unwrap mutex");
                if let Some(mut child) = value.take() {
                    child.kill()
                        .expect("failed to wait on child");
                }
                *value = Some(start_audio(&fp, 50));
            });
        }),
    );

    let data2 = data.clone();
    let fp = file_path.clone();
    bar.add_item(
        "Play 25%",
        Box::new(move || {
            let data2 = data2.clone();
            let fp = fp.clone();
            spawn(move || {
                let mut value = data2.lock().expect("could not unwrap mutex");
                if let Some(mut child) = value.take() {
                    child.kill()
                        .expect("failed to wait on child");
                }
                *value = Some(start_audio(&fp, 25));
            });
        }),
    );

    let data2 = data.clone();
    bar.add_item(
        "Stop",
        Box::new(move || {
            let data2 = data2.clone();
            spawn(move || {
                println!("KILLING");
                let mut value = data2.lock().expect("could not unwrap mutex");
                if let Some(mut child) = value.take() {
                    child.kill()
                        .expect("failed to wait on child");
                }
            });
        }),
    );


    let data2 = data.clone();
    bar.add_item(
        "Quit",
        Box::new(move || {
            let data2 = data2.clone();
            spawn(move || {
                println!("KILLING");
                let mut value = data2.lock().expect("could not unwrap mutex");
                if let Some(mut child) = value.take() {
                    child.kill()
                        .expect("failed to wait on child");
                }
                ::std::process::exit(0);
            });
        }),
    );

    bar.display();
}

fn start_audio(file_path: &Path, level: i32) -> Child {
    let play = Command::new("play")
        .arg("-t").arg("mp3")
        .arg(file_path)
        .arg("trim").arg("20").arg("140")
        .arg("repeat")
        .arg("-")
        .arg("vol")
        .arg(format!("{}", (level as f32) / 100.0))
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    // if let Some(ref mut stdin) = play.stdin {
    //     stdin.write(RAIN);
    // }`
    
    play
}
