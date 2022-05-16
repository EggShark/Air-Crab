#![allow(non_snake_case)]

use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;

mod song;
use song::Song;

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
                    "exit" => {
                        break;
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

fn play(file_name: &str){
    // this will be way harder
    // find file
    // async play ---> may require a second channel :)
    // pause when pause is entered
    let file_path = String::from("music/") + file_name;
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

    let mut file_size: [u8; 4] = [0;4];
    reader.read_exact(&mut file_size[..]).unwrap();
    let file_size = u32::from_le_bytes(file_size);
    println!("{}", file_size);

    let mut format: [u8; 4] = [0;4];
    reader.read_exact(&mut format[..]).unwrap();
    if b"WAVE" == &format {
        //do stuff
        println!("an audio file");
    }
    else {
        panic!("NOT A WAVE FILE");
    }
    let song = song_constructor(&mut reader);
    println!("{:?}",song);
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

fn song_constructor(file: &mut BufReader<File>) -> Song{
    //read fmt chunk and create struct
    let mut fmt_header: [u8; 4] = [0;4];
    file.read_exact(&mut fmt_header[..]).unwrap();
    if &fmt_header != b"fmt " {
        panic!("header not found");
    }
    // gets size of fmt chunk
    let mut fmt_size: [u8;4] = [0;4];
    file.read_exact(&mut fmt_size[..]).unwrap();
    let _fmt_size = u32::from_le_bytes(fmt_size);

    let mut audio_format: [u8;2] = [0;2];
    file.read_exact(&mut audio_format[..]).unwrap();
    let audio_format = u16::from_le_bytes(audio_format);
    // need to process that 1 = pcm any other number indicates some kind of compression

    let mut channel_num: [u8;2] =[0;2];
    file.read_exact(&mut channel_num[..]).unwrap();
    let channel_num = u16::from_le_bytes(channel_num);
    // turn into u32 1 = mono 2 = stero;

    let mut sample_rate: [u8;4] = [0;4];
    file.read_exact(&mut sample_rate[..]).unwrap();
    let sample_rate = u32::from_le_bytes(sample_rate);
    // turn into u32

    let mut byte_rate: [u8;4] = [0;4];
    file.read_exact(&mut byte_rate[..]).unwrap();
    let byte_rate = u32::from_le_bytes(byte_rate);
    // another u32 not sure why this exists

    let mut block_align: [u8;2] = [0;2];
    file.read_exact(&mut block_align[..]).unwrap();
    let block_align = u16::from_le_bytes(block_align);
    // a u16 The number of bytes for one sample including all channels.

    let mut bits_per_sample: [u8;2] = [0;2];
    file.read_exact(&mut bits_per_sample[..]).unwrap();
    let bits_per_sample = u16::from_le_bytes(bits_per_sample);
    // if its an 8 = 8bits 16=16bit so on

    let mut extra_perams_size: [u8;2] = [0;2];
    file.read_exact(&mut extra_perams_size[..]).unwrap();
    let extra_perams_size = u16::from_le_bytes(extra_perams_size);
    //this shouldn't exist if its pcm but data shows otherwize but its the size in bytes simillar to fmt chunk size

    let processed_song = Song{
        audio_format: audio_format,
        channels: channel_num,
        sample_rate: sample_rate,
        byte_rate: byte_rate,
        block_align: block_align,
        bits_per_sample: bits_per_sample,
        extra_perams_size: extra_perams_size
    };

    processed_song
}