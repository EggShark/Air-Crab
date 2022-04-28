#![allow(non_snake_case)]

use std::io;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // let mut pain = String::from("a");
    // println!("{}", pain.as_str());
    // pain.push_str(" boy");
    // pain.pop();
    // assert_eq!(pain.as_str(), "a boy");


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
                    if file.ends_with("\r"){
                        file.pop();
                    }
                }
                match file.as_str() {
                    "play" => {
                        play();
                    },
                    "pause" => {
                        pause();
                    },
                    _ => {
                        println!("catch all you {} {}", file.as_str(), file.as_str() == "e");
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