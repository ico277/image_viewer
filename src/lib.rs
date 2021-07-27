#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use image::ImageBuffer;
use std::iter::Iterator;

pub const VERSION: &str = "2.1.1";

pub struct Settings {
    pub image_file: String,
    pub downscale_ratio_per_pix: f32,
    pub gif_loop: bool,
}

pub fn parse_file(settings: &mut Settings, iter: &mut dyn Iterator<Item = &str>) {
    for i in iter {
        match i.split_once("=") {
            Some(("img", s)) => {
                settings.image_file = s.to_string();
            }
            Some(("downscale_ratio", s)) => match String::from(s).parse() {
                Ok(r) => settings.downscale_ratio_per_pix = r,
                Err(err) => {
                    eprintln!(
                        "Warning: Unable to parse downscale_ratio in the settings: {}",
                        err
                    );
                    eprintln!("Warning: Using default amount of {}", 1.5_f32);
                    settings.downscale_ratio_per_pix = 1.5_f32;
                }
            },
            Some(("gif_loop", s)) => match s {
                "true" => settings.gif_loop = true,
                "false" => settings.gif_loop = false,
                _ => (),
            },
            None => break,
            _ => continue,
        }
    }
}

pub fn parse_args(settings: &mut Settings, iter: &mut dyn Iterator<Item = String>) {
    //Iterates the iterator once to remove the first argument
    iter.next();

    //Match the first argument as the file
    match iter.next().unwrap_or(String::from(" ")).as_str() {
        "-v" | "--version" => {
            println!("image_viewer v{}", VERSION);
            std::process::exit(0);
        },
        " " => (),
        s => {
            settings.image_file = String::from(s);
        }
    }

    for i in iter {
        match i.split_once("=") {
            Some(("img", s)) => {
                settings.image_file = s.to_string();
            }
            Some(("downscale_ratio", s)) => match s.parse() {
                Ok(r) => settings.downscale_ratio_per_pix = r,
                Err(err) => {
                    eprintln!("Warning: Unable to parse downscale_ratio: {}", err);
                    eprintln!("Warning: Using default amount of {}", 2_f32);
                    settings.downscale_ratio_per_pix = 2_f32;
                }
            },
            Some(("gif_loop", s)) => match s {
                "true" => settings.gif_loop = true,
                "false" => settings.gif_loop = false,
                _ => (),
            },
            None => break,
            _ => continue,
        }
    }
}

/*fn resize_pluto(width: i32, height: i32) -> pluto_lib_t {
    unsafe {
        _pluto_canvas.cheight = height;
        _pluto_canvas.height = height / 4;

        _pluto_canvas.cwidth = width;
        _pluto_canvas.width = width / 2;

        //pluto_resize();

        _pluto_canvas
    }
}*/

pub fn display_image(img: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>) {
    unsafe {
        //resize_pluto(img.width() as i32, img.height() as i32);
        //println!("cheight:{} cwidth:{}", _pluto_canvas.cheight, _pluto_canvas.cwidth);
        //println!("height:{} width:{}", img.height(), img.width());
        for x in 0..img.width() as u32 {
            //Loops through every pixel in the row
            for y in 0..img.height()  as u32 {
                //Gets color data and prints it
                let color = img.get_pixel(x, y).0;
                pluto_set_cpix(x as i32, y as i32, color[0], color[1], color[2]);
            }
        }
        pluto_write_out();
        pluto_render();
    }
}

pub fn clean_exit() {
    // Deinits pluto
    unsafe {
        pluto_deinit();
    }
    //Resets the color
    print!("\x1b[0m");
}