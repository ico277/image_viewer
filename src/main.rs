use image_viewer::*;

use image::imageops::resize;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::PathBuf;

use dirs::home_dir;

struct Settings {
    image_file: String,
    symbol: String,
    downscale_ratio_per_char: f32,
}

fn parse_file(settings: &mut Settings, iter: &mut dyn Iterator<Item = &str>) {
    for i in iter {
        match i.split_once("=") {
            Some(("img", s)) => {
                settings.image_file = s.to_string();
            }
            Some(("symbol", s)) => {
                settings.symbol = s.to_string();
            }
            Some(("downscale_ratio", s)) => match String::from(s).parse() {
                Ok(r) => settings.downscale_ratio_per_char = r,
                Err(err) => {
                    eprintln!(
                        "Warning: Unable to parse downscale_ratio in the settings: {}",
                        err
                    );
                    eprintln!("Warning: Using default amount of {}", 1.5_f32);
                    settings.downscale_ratio_per_char = 1.5_f32;
                }
            },
            None => break,
            _ => continue,
        }
    }
}

fn parse_args(settings: &mut Settings, iter: &mut dyn Iterator<Item = String>) {
    //Iterates the iterator once to remove the first argument
    iter.next();

    //Match the first argument as the file
    match iter.next() {
        Some(s) => {
            settings.image_file = s;
        }
        None => (),
    }

    for i in iter {
        match i.split_once("=") {
            Some(("img", s)) => {
                settings.image_file = s.to_string();
            }
            Some(("symbol", s)) => {
                settings.symbol = s.to_string();
            }
            Some(("downscale_ratio", s)) => match s.parse() {
                Ok(r) => settings.downscale_ratio_per_char = r,
                Err(err) => {
                    eprintln!("Warning: Unable to parse downscale_ratio: {}", err);
                    eprintln!("Warning: Using default amount of {}", 2_f32);
                    settings.downscale_ratio_per_char = 2_f32;
                }
            },
            None => break,
            _ => continue,
        }
    }
}

fn main() {
    std::io::stdout();
    unsafe {
        pluto_init_window();
    }
    let mut settings = Settings {
        image_file: String::new(),
        symbol: String::from("##"), // used to be "■■"
        downscale_ratio_per_char: 2_f32,
    };
    let term_size;
    unsafe {
        term_size = [_pluto_canvas.cwidth as f32, _pluto_canvas.cheight as f32];
    }
    //let mut downscale_ratio_per_char = 2;

    //Parsing config file
    let mut user_dir = match home_dir() {
        Some(s) => s,
        _ => PathBuf::new(),
    };
    if user_dir.as_os_str().is_empty() {
        eprintln!("Warning: Couldn't load config file: Couldn't get user folder");
        eprintln!("Warning: using default config");
    } else {
        user_dir.push(".image_viewer");
        if !user_dir.as_path().exists() {
            match user_dir.as_path().to_str() {
                Some(s) => match fs::create_dir(s) {
                    Ok(()) => {
                        user_dir.push("config.txt");
                        match File::open(&user_dir) {
                            Ok(mut file) => {
                                let mut file_str = String::new();
                                match file.read_to_string(&mut file_str) {
                                    Ok(_) => {
                                        parse_file(&mut settings, &mut file_str.lines());
                                    }
                                    Err(err) => {
                                        eprintln!("Warning: Couldn't load config file: {}", err);
                                        eprintln!("Warning: using default config");
                                    }
                                }
                            }
                            Err(_) => match File::create(user_dir) {
                                Ok(mut file) => match file.write_all(b"symbol=##\n") {
                                    Ok(()) => (),
                                    Err(err) => {
                                        eprintln!(
                                            "Warning: Couldn't write to config file: {}",
                                            err
                                        );
                                        eprintln!("Warning: using default config");
                                    }
                                },
                                Err(err) => {
                                    eprintln!("Warning: Couldn't create config file: {}", err);
                                    eprintln!("Warning: using default config");
                                }
                            },
                        }
                    }
                    Err(err) => {
                        eprintln!("Warning: Couldn't load config file: {}", err);
                        eprintln!("Warning: using default config");
                    }
                },
                None => {
                    eprintln!("Warning: Couldn't load config file: Couldn't get/create .image_viewer folder");
                    eprintln!("Warning: using default config");
                }
            }
        } else {
            user_dir.push("config.txt");
            match File::open(&user_dir) {
                Ok(mut file) => {
                    let mut file_str = String::new();
                    match file.read_to_string(&mut file_str) {
                        Ok(_) => {
                            parse_file(&mut settings, &mut file_str.lines());
                        }
                        Err(err) => {
                            eprintln!("Warning: Couldn't load config file: {}", err);
                            eprintln!("Warning: using default config");
                        }
                    }
                }
                Err(_) => match File::create(user_dir) {
                    Ok(mut file) => match file.write_all(b"symbol=##\n") {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!("Warning: Couldn't write to config file: {}", err);
                            eprintln!("Warning: using default config");
                        }
                    },
                    Err(err) => {
                        eprintln!("Warning: Couldn't create config file: {}", err);
                        eprintln!("Warning: using default config");
                    }
                },
            }
        }
    }

    //Parsing args
    parse_args(&mut settings, &mut env::args());
    //println!("Settings: {{\n\timage_file: '{}'\n\tsymbol: '{}'\n\tdownscale_ratio: '{}'\n}}", settings.image_file, settings.symbol, settings.downscale_ratio_per_char);

    //Reading image file
    let img = match ImageReader::open(settings.image_file) {
        Ok(img) => match img.decode() {
            Ok(img) => img,
            Err(err) => {
                eprintln!(
                    "There was an error whilst trying to decode the image file: {}",
                    err
                );
                std::process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("There was an error whilst trying to read the file: {}", err);
            std::process::exit(1);
        }
    };
    //Clears console and moves the curser to the start
    print!("\x1b[2J\x1b[H");

    //Gets image data and dimensions of the image
    let img = img.into_rgb8();
    let img_dimensions = [img.width() as f32, img.height() as f32];

    //NOTE downScaleRatio = (termWidth/fullImgWidth); resize(img, fullImgWidth*downScaleRatio, fullImgHeight*downScaleRatio);
    //Calculates the downscale ratio
    let downscale_ratio = term_size[0] / img_dimensions[0];
    let new_width = (img_dimensions[0] * downscale_ratio) / settings.downscale_ratio_per_char;
    let new_height = (img_dimensions[1] * downscale_ratio) / settings.downscale_ratio_per_char;

    //Downscales the image and displays the new size of the image
    let img = resize(
        &img,
        new_width.floor() as u32,
        new_height.floor() as u32,
        FilterType::Nearest,
    );
    let img_dimensions_downscaled = [img.width(), img.height()];
    println!(
        "{}p x {}p (downscaled to {}p x {}p)",
        img_dimensions[0],
        img_dimensions[1],
        img_dimensions_downscaled[0],
        img_dimensions_downscaled[1]
    );

    /*
    //Loops through every row of the image
    for y in 0..img.height() {
        //Loops through every pixel in the row
        for x in 0..img.width() {
            //Gets color data and prints it
            let color = img.get_pixel(x, y).0;
            print!("\x1b[38;2;{};{};{}m{}", color[0], color[1], color[2], settings.symbol);
        }
        //Prints a new line after a row
        println!();
    }
    //Resets the color
    print!("\x1b[0m");
    */
    unsafe {
        for y in 0..img.height() {
            //Loops through every pixel in the row
            for x in 0..img.width() {
                //Gets color data and prints it
                let color = img.get_pixel(x, y).0;
                //print!("\x1b[38;2;{};{};{}m{}", color[0], color[1], color[2], settings.symbol);
                pluto_set_cpix(x as i32, y as i32, color[0], color[1], color[2]);
            }
        }
        pluto_write_out();
        pluto_render();
        pluto_deinit();
        //Resets the color
        print!("\x1b[0m");
    }
}
