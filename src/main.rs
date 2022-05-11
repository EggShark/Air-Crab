#![allow(non_snake_case)]

use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;

fn main() {
    let (send, recv) = channel();
    thread::spawn(move || {
        loop {
            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("failed to read line");
            send.send(message);
        }
    });
//-------------------- thread 1 ^ thread 2 bellow;
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
                let args = args_spliter(file);
                match args[0].as_str() {
                    "play" => {
                        // makes sure the user specified something to play
                        if args.len() < 2 {
                            println!("no Song speicifed");
                        }
                        else{
                            play(args[1].as_str());
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
    // this will be way harder
    // find file
    // async play ---> may require a second channel :)
    // pause when pause is entered
    let file_path = String::from("music/") + fileName;
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut header: [u8; 4] = [0; 4];
    reader.read_exact(&mut header[..]).unwrap();
    println!("data in buffer {:?}", header);
    if &header == b"RIFF" {
        println!("its a RIFF");
    }
    else {
        println!("not a RIFF");
    }
    let mut wave_fmt: [u8; 7] = [0;7];
    reader.read_exact(&mut wave_fmt[8..]).unwrap();
    assert_eq!(b"WAVEfmt", &wave_fmt);
}

fn pause(){
    // make a value true --> false then back to true while sending it to a diffrent thread or somehow notifying the player
    println!("pause");
}

fn args_spliter(arg: String) -> Vec<String>{
    let mut args = Vec::new();
    let mut chars = arg.chars();
    
    while let Some(letter) = chars.next() {
        args.push(match letter {
            ' ' => continue,
            quote @ ('\"' | '\'') => chars.by_ref().take_while(|ch| ch != &quote).collect(),
            c => String::from(c) + &chars.by_ref().take_while(|ch| ch != &' ').collect::<String>(),
        });
    }
    args
}