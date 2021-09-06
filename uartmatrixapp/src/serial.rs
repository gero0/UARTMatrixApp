use std::io::{Read, Write};
use std::{thread, time};

use image::RgbImage;
use serialport::SerialPort;

use libuartmatrix::enums::DisplayMode;
use libuartmatrix::*;

use crate::helper_structs::{Animation, Direction, Font, RgbColor};

pub fn send_text(port: &mut dyn SerialPort, text_rows: &[String]) {
    for (i, row) in text_rows.iter().enumerate() {
        if row.len() == 0 {
            continue;
        }
        let result = serialize_write_line(i as u8, row.as_str());
        if let Some(packet) = result {
            port.write(&packet);
            let mut buffer = [0; 20];
            port.read(&mut buffer);
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}

pub fn send_colors(port: &mut dyn SerialPort, color_rows: &[RgbColor]) {
    for (i, row) in color_rows.iter().enumerate() {
        let result = serialize_set_color(i as u8, row.r as u8, row.g as u8, row.b as u8);
        if let Some(packet) = result {
            port.write(&packet);
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}

pub fn send_animations(
    port: &mut dyn SerialPort,
    animations: &[Option<Animation>],
    animation_speeds: &[String],
    animation_directions: &[Option<Direction>],
) {
    for (i, ((anim, speed), direction)) in animations
        .iter()
        .zip(animation_speeds)
        .zip(animation_directions)
        .enumerate()
    {
        if anim.is_none() {
            continue;
        }

        let animation = match anim.unwrap() {
            Animation::Blink => {
                let speed = str::parse::<u8>(speed);
                if speed.is_err() {
                    continue;
                }

                libuartmatrix::enums::Animation::BlinkAnimation(speed.unwrap())
            }
            Animation::Slide => {
                let speed = str::parse::<u8>(speed);
                if speed.is_err() {
                    continue;
                }

                if direction.is_none() {
                    continue;
                }

                let dir = match direction.unwrap() {
                    Direction::Left => libuartmatrix::enums::Direction::Left,
                    Direction::Right => libuartmatrix::enums::Direction::Right,
                };

                libuartmatrix::enums::Animation::SlideAnimation(speed.unwrap(), dir)
            }
            Animation::None => libuartmatrix::enums::Animation::NoAnimation,
        };

        let result = serialize_set_animation(i as u8, animation);
        if let Some(packet) = result {
            port.write(&packet);
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}

pub fn send_fonts(port: &mut dyn SerialPort, fonts: &[Option<Font>]) {
    for (i, row_font) in fonts.iter().enumerate() {
        if row_font.is_none() {
            continue;
        }

        let font = match row_font.unwrap() {
            Font::Default => libuartmatrix::enums::FontType::Default,
            Font::Ibm => libuartmatrix::enums::FontType::Ibm,
            Font::Pro => libuartmatrix::enums::FontType::Pro,
        };

        let result = serialize_set_font(i as u8, font.into());

        if let Some(packet) = result {
            port.write(&packet);
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}

pub fn send_change_mode(port: &mut dyn SerialPort, mode: DisplayMode) {
    let result = serialize_switch_mode(mode);
    if let Some(packet) = result {
        port.write(&packet);
        thread::sleep(time::Duration::from_millis(20));
    }
}

pub fn send_image(port: &mut dyn SerialPort, image: RgbImage) {
    for (i, row) in image.rows().enumerate() {
        let mut row_vec: Vec<(u8, u8, u8)> = Vec::new();
        for pixel in row {
            let pixel = pixel.0;
            let pixel = (pixel[0], pixel[1], pixel[2]);
            row_vec.push(pixel);
        }

        let result = serialize_draw_row(i as u8, row_vec);
        if let Some(packet) = result {
            port.write(&packet);
            thread::sleep(time::Duration::from_millis(50));
        }
    }
}
