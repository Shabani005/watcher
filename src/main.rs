use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let program: &String = &args[0];
        println!("Usage: {program} <file>");
        return;
    }

    let mut seen_plus: bool = false;
    let mut plus_loc = None;

    let mut files = Vec::new();

    for (i, arg) in args.iter().enumerate().skip(1) {
        if arg == "+" {
            seen_plus = true;
            plus_loc = Some(i);
            break;
        }

        if !fs::exists(&arg).unwrap(){
            println!("File {arg} does not exist!");
        } else {
            files.push(arg);
        } 
    }

    let mut contents = Vec::new();    

    for file in &files {
        let mut content = String::new();
        let mut f = File::open(file).unwrap();
        f.read_to_string(&mut content).unwrap();
        contents.push(content.clone());
    }

    const SECS: u64 = 1;
    println!("Watching for change in files every {SECS} seconds");

    while true {
        for i in 0..contents.len() {
            let mut content = String::new();
            let mut f = File::open(&files[i]).unwrap();
            f.read_to_string(&mut content).unwrap();
            if contents[i] != content {
                println!("File changed!");
                contents[i] = content;

                let mut cmd = Vec::new();
                for j in plus_loc.unwrap() + 1..args.len(){
                    cmd.push(&args[j]);
                }
                Command::new(cmd[0])
                    .args(&cmd[1..])
                    .status()
                    .unwrap();
            }  
        }
        thread::sleep(time::Duration::from_secs(SECS));
    }

}
