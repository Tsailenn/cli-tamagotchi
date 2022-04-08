mod tamagotchi;

use std::io;
use std::mem::replace;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

fn main() {
    let stdin_channel = spawn_stdin_channel();
    let mut inst = tamagotchi::Tamagotchi::new();
    let mut frame: u128 = 0;
    inst.draw();

    loop {
        println!("iterations: {:#?}", frame);

        inst.update(-1, -1, 2);
        match stdin_channel.try_recv() {
            Ok(key) => {
                // println!("Received: {}", key);
                let mut sanitized_key = key.clone();
                sanitized_key = sanitized_key.replace("\n", "").replace("\r", "");
                println!("Sanitized key: {:?}", sanitized_key);

                for c in sanitized_key.chars() {
                    let mut fe = 0;
                    let mut cu = 0;
                    match c {
                        'f' => {
                            println!("feeding");
                            fe = 5;
                        }
                        'c' => {
                            println!("curing");
                            cu = 7;
                        }
                        _ => {}
                    }
                    inst.update(fe, cu, 0);
                }
            }
            Err(TryRecvError::Empty) => println!("No actions taken"),
            Err(TryRecvError::Disconnected) => panic!("Ended"),
        }
        inst.print();
        if inst.is_dead() {
            break;
        }
        sleep(5000);
        frame += 1;
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
