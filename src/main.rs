#![allow(non_snake_case)]

use std::io;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    println!("{:?}", "Hello there general".chars().position(|c| c == ' ').unwrap());

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
            Ok(mut file) => {
                if file.ends_with("\n") {
                    file.pop();
                    if file.ends_with("\r"){
                        file.pop();
                    }
                }
                // split into vector
                let args = args_spliter(file.as_str()); 
                match args[0] {
                    "play" => {
                        play();
                    },
                    "pause" => {
                        pause();
                    },
                    _ => {
                        println!("not a command");
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

fn args_spliter(arg: &str) -> Vec<&str>{
    let mut vector = Vec::new();
    let mut lastSpace = 0;
    let mut i = 0;
    loop {
        if i >= arg.len() {
            break;
        }
        let letter = arg.chars().nth(i).unwrap();
        if letter == '\"'{
            let split = &arg[i+1..];
            let endQoute = split.find("\"");
            let endQoute = match endQoute{
                Some(x) => x,
                None => panic!("No End Qoute"), // temperarry error handling need more robust later
            };
            vector.push(&split[..endQoute]);
            lastSpace = i + endQoute + 3; // assumes there is a space after qoutes could cuase many bugs
            i += endQoute + 3; // I fixed a bug but now it works dont touch this line
        }
        else if letter == '\''{
            let split = &arg[i+1..];
            let endQoute = split.find("\'");
            let endQoute = match endQoute{
                Some(x) => x,
                None => panic!("No End Qoute"), // temperarry error handling need more robust later
            };
            vector.push(&split[..endQoute]);
            lastSpace = i + endQoute + 3; // assumes there is a space after qoutes could cuase many bugs
            i += endQoute + 3; // I fixed a bug but now it works dont touch this line
        }
        else if letter == ' '{
            vector.push(&arg[lastSpace..i]);
            lastSpace = i+1;
        }
        i += 1;
    }
    vector.push(&arg[lastSpace..]);
    println!("{:?}", vector);
    vector
}