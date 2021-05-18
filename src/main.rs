use image::io::Reader as ImageReader;

use std::env;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::path::PathBuf;

use dirs::home_dir;

fn main() {
    let mut file = String::new();
    let mut symbol = "##".to_string();// used to be "■■".to_string();
    
    //Clears console
    print!("\x1b[2J\x1b[H");

    //Parsing config file
    let mut user_dir = match home_dir() {
        Some(s) => s,
        _ => PathBuf::new(),
    };
    if user_dir.as_os_str().is_empty() {
        println!("Warning: Couldn't load config file: Couldn't get user folder");
        println!("Warning: using default config");
    } else {
        user_dir.push(".image_viewer");
        if !user_dir.as_path().exists() {
            match user_dir.as_path().to_str() {
                Some(s) => {
                    match fs::create_dir(s) {
                        Ok(()) => {
                            user_dir.push("config.txt");
                            match File::open(&user_dir) {
                                Ok(mut file) => {
                                    let mut file_str = String::new();
                                    match file.read_to_string(&mut file_str) {
                                        Ok(_) => {
                                            for line in file_str.lines() {
                                                match line.split_once("=") {
                                                    Some(("symbol", s)) => {
                                                        symbol = s.to_string();
                                                    },
                                                    _ => continue,
                                                }
                                            }
                                        },
                                        Err(err) => {
                                            println!("Warning: Couldn't load config file: {}", err);
                                            println!("Warning: using default config");
                                        },
                                    }
                                },
                                Err(_) => {
                                    match File::create(user_dir) {
                                        Ok(mut file) => {
                                            match file.write_all(b"symbol=##\n") {
                                                Ok(()) => (),
                                                Err(err) => {
                                                    println!("Warning: Couldn't write to config file: {}", err);
                                                    println!("Warning: using default config");
                                                },
                                            }
                                        },
                                        Err(err) => {
                                            println!("Warning: Couldn't create config file: {}", err);
                                            println!("Warning: using default config");
                                        },
                                    }
                                },
                            }
                        },
                        Err(err) => {
                            println!("Warning: Couldn't load config file: {}", err);
                            println!("Warning: using default config");
                        }
                    }
                },
                None => {
                    println!("Warning: Couldn't load config file: Couldn't get/create .image_viewer folder");
                    println!("Warning: using default config");
                },
            }
        } else {
            user_dir.push("config.txt");
            match File::open(&user_dir) {
                Ok(mut file) => {
                    let mut file_str = String::new();
                    match file.read_to_string(&mut file_str) {
                        Ok(_) => {
                            for line in file_str.lines() {
                                match line.split_once("=") {
                                    Some(("symbol", s)) => {
                                        symbol = s.to_string();
                                    },
                                    _ => continue,
                                }
                            }
                        },
                        Err(err) => {
                            println!("Warning: Couldn't load config file: {}", err);
                            println!("Warning: using default config");
                        },
                    }
                },
                Err(_) => {
                    match File::create(user_dir) {
                        Ok(mut file) => {
                            match file.write_all(b"symbol=##\n") {
                                Ok(()) => {

                                },
                                Err(err) => {
                                    println!("Warning: Couldn't write to config file: {}", err);
                                    println!("Warning: using default config");
                                },
                            }
                        },
                        Err(err) => {
                            println!("Warning: Couldn't create config file: {}", err);
                            println!("Warning: using default config");
                        },
                    }
                },
            }
        }
    }

    //Parsing args
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(s) => {
            file = s;
        },
        None => (),
    }
    for arg in args {
        match arg.split_once("=") {
            Some(("img", s)) => {
                file = s.to_string();
            },
            Some(("symbol", s)) => {
                symbol = s.to_string();
            },
            _ => (),
        }
    }

    //Reading image file
    let img = match ImageReader::open(file) {
        Ok(img) => match img.decode() {
            Ok(img) => img,
            Err(err) => {
                println!("There was an error whilst trying to decode the image file: {}", err);
                std::process::exit(1);
            },
        },
        Err(err) => {
            println!("There was an error whilst trying to read the file: {}", err);
            std::process::exit(1);
        },
    };
    
    //Get image data and prints the dimension of the image 
    let img = img.into_rgb8();
    println!("{}p x {}p", img.width(), img.height());

    //Loops through every row of the image
    for y in 0..img.height() {
        //Loops through every pixel in the row
        for x in 0..img.width() {
            //Gets color data and prints it
            let color = img.get_pixel(x, y).0;
            print!("\x1b[38;2;{};{};{}m{}", color[0], color[1], color[2], symbol);
        }
        //Prints a new line after a row
        println!();
    }
}