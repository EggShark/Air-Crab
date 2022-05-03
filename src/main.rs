#![allow(non_snake_case)]

use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::fs;

fn main() {
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
                let args = args_spliter(file.as_str());
                match args[0] {
                    "play" => {
                        // makes sure the user specified something to play
                        if args.len() < 2 {
                            println!("no Song speicifed");
                        }
                        else{
                            play(args[1]);
                        }
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

fn play(fileName: &str){
    // find file
    // async play ---> may require a second channel :)
    // pause when pause is entered
    let filePath = String::from("music/") + fileName;
    let contents = fs::read_to_string(filePath).expect("Something went wrong reading the file"); 
    println!("play {}", contents);
}

fn pause(){
    // make a value true --> false then back to true while sending it to a diffrent thread or somehow notifying the player
    println!("pause");
}

fn args_spliter(arg: &str) -> Vec<&str>{
    let mut vector = Vec::new();
    let mut last_space = 0;
    let mut i = 0;
    loop {
        if i >= arg.len() {
            break;
        }
        let letter = arg.chars().nth(i).unwrap();
        if letter == '\"'{
            let split = &arg[i+1..];
            let end_qoute = split.find("\"");
            let end_qoute = match end_qoute{
                Some(x) => x,
                None => panic!("No End Qoute"), // temperarry error handling need more robust later
            };
            vector.push(&split[..end_qoute]);
            last_space = i + end_qoute + 3; // assumes there is a space after qoutes could cuase many bugs
            i += end_qoute + 3; // I fixed a bug but now it works dont touch this line
        }
        else if letter == '\''{
            let split = &arg[i+1..];
            let end_qoute = split.find("\'");
            let end_qoute = match end_qoute{
                Some(x) => x,
                None => panic!("No End Qoute"), // temperarry error handling need more robust later
            };
            vector.push(&split[..end_qoute]);
            last_space = i + end_qoute + 3; // assumes there is a space after qoutes could cuase many bugs
            i += end_qoute + 3; // I fixed a bug but now it works dont touch this line
        }
        else if letter == ' '{
            vector.push(&arg[last_space..i]);
            last_space = i+1;
        }
        i += 1;
    }
    vector.push(&arg[last_space..]);
    vector
}