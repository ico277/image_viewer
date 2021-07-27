use image_viewer::*;

use image::gif::GifDecoder;
use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;
use image::AnimationDecoder;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;
use std::{thread, time};

use dirs::home_dir;

fn main() {
    ctrlc::set_handler(move || clean_exit()).expect("Error setting Ctrl-C handler");
    std::io::stdout();

    let mut settings = Settings {
        image_file: String::new(),
        downscale_ratio_per_pix: 2_f32,
        gif_loop: false,
    };

    let term_size;

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
                                Ok(mut file) => match file.write_all(b"gif_loop=true\n") {
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
                    Ok(mut file) => match file.write_all(b"gif_loop=true\n") {
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
    //println!("Settings: {{\n\timage_file: '{}'\n\tsymbol: '{}'\n\tdownscale_ratio: '{}'\n}}", settings.image_file, settings.symbol, settings.downscale_ratio_per_pix);
    //Checking if image file is an animated GIF
    if settings.image_file.to_lowercase().ends_with(".gif") {
        // Decode a gif into frames
        let file_in = match File::open(settings.image_file) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("There was an error whilst trying to read the file: {}", err);
                std::process::exit(1);
            }
        };
        let decoder = match GifDecoder::new(file_in) {
            Ok(m) => m,
            Err(err) => {
                eprintln!(
                    "There was an error whilst trying to decode the image file: {}",
                    err
                );
                std::process::exit(1);
            }
        };
        unsafe {
            pluto_init_window();
            term_size = [_pluto_canvas.cwidth as f32, _pluto_canvas.cheight as f32];
        }
        let frames = decoder.into_frames();
        let frames = match frames.collect_frames() {
            Ok(m) => m,
            Err(err) => {
                eprintln!(
                    "There was an error whilst trying to decode the image file: {}",
                    err
                );
                std::process::exit(1);
            }
        };
        if settings.gif_loop {
            //let frames_len = frames.len();
            let mut frames2 = frames.to_owned();
            loop {
                for frame in frames2 {
                    let time = time::Duration::from_millis(frame.delay().numer_denom_ms().0 as u64);
                    let mut img = frame.into_buffer();
                    let img_dimensions = [img.width() as f32, img.height() as f32];
                    if img_dimensions[0] <= term_size[0] && img_dimensions[1] <= term_size[1] {
                        let downscale_ratio = term_size[0] / img_dimensions[0];
                        let new_width =
                            img_dimensions[0] * downscale_ratio;
                        let new_height =
                            img_dimensions[1] * downscale_ratio;
                        img = resize(
                            &img,
                            new_width.floor() as u32,
                            new_height.floor() as u32,
                            FilterType::Nearest,
                        );
                    }
                    display_image(img);
                    thread::sleep(time);
                }
                frames2 = frames.to_owned();
            }
        } else {
            for frame in frames {
                let time = time::Duration::from_millis(frame.delay().numer_denom_ms().0 as u64);
                let mut img = frame.into_buffer();
                let img_dimensions = [img.width() as f32, img.height() as f32];
                if img_dimensions[0] <= term_size[0] && img_dimensions[1] <= term_size[1] {
                    let downscale_ratio = term_size[0] / img_dimensions[0];
                    let new_width =
                        img_dimensions[0] * downscale_ratio;
                    let new_height =
                        img_dimensions[1] * downscale_ratio;
                    img = resize(
                        &img,
                        new_width.floor() as u32,
                        new_height.floor() as u32,
                        FilterType::Nearest,
                    );
                }
                display_image(img);
                thread::sleep(time);
            }
        }
    } else {
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
        unsafe {
            pluto_init_window();
            term_size = [_pluto_canvas.cwidth as f32, _pluto_canvas.cheight as f32];
        }

        //Gets image data and dimensions of the image
        let mut img = img.into_rgba8();
        let img_dimensions = [img.width() as f32, img.height() as f32];

        //NOTE downScaleRatio = (termWidth/fullImgWidth); resize(img, fullImgWidth*downScaleRatio, fullImgHeight*downScaleRatio);
        //Calculates the downscale ratio
        //println!("{}p x {}p", term_size[0], term_size[1]);
        if  term_size[0] < img_dimensions[0] || term_size[1] < img_dimensions[1] {
            let downscale_ratio = term_size[0] / img_dimensions[0];
            let new_width =
                img_dimensions[0] * downscale_ratio;
            let new_height =
                img_dimensions[1] * downscale_ratio;
            img = resize(
                &img,
                new_width.floor() as u32,
                new_height.floor() as u32,
                FilterType::Nearest,
            );
        }
        let img_dimensions_downscaled = [img.width(), img.height()];
        println!(
            "{}p x {}p (downscaled to {}p x {}p)",
            img_dimensions[0],
            img_dimensions[1],
            img_dimensions_downscaled[0],
            img_dimensions_downscaled[1]
        );
        display_image(img);
    }
    clean_exit();
}
