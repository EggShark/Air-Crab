#![allow(non_snake_case)]

use std::io;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let mut pain = String::from("a");
    println!("{}", pain.as_str());
    pain.push_str(" boy");
    pain.pop();
    println!("why do you break {} , {}", pain.as_str(), {pain.as_str() == "a bo"});


    let (send, recv) = channel();
    thread::spawn(move || {
        loop {
            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("failed to read line");
            send.send(message);
        }
    });

    loop {
        let a = recv.try_recv();
        match a {
            Ok(mut file) => {if file.ends_with("\n"
                ){
                    file.pop();
                }
                let str_file: &str = file.as_str();
                println!("before match {} {}", str_file, str_file == "a");
                match str_file {
                    "play" => {
                        play();
                    },
                    "pause" => {
                        pause();
                    },
                    _ => {
                        println!("catch all you {} {}", str_file, str_file == "e");
                    },
                };
            },
            Err(_error) => {},
        };
    }
}

fn play(){
    println!("play");
}

fn pause(){
    println!("pause");
}