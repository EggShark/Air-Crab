#![allow(non_snake_case)]

use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (send, recv) = channel();
    thread::spawn(move || {
        loop {
            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("failed to read line");
            send.send(message);
        }
    });

    let mut lastMessage = String::from("a");
    loop {
        let a = recv.try_recv();
        let a = match a {
            Ok(mut file) => {
                if file.ends_with("\n"){
                    file.pop();
                }
                lastMessage = file;
                &lastMessage
            },
            Err(_error) => {
                &lastMessage
            },
        };
        println!("{}", a);
        thread::sleep(Duration::from_secs(2));
    }
}
