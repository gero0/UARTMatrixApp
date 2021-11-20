use std::{thread, time};

use image::RgbImage;
use serialport::SerialPort;

use libuartmatrix::enums::DisplayMode;
use libuartmatrix::*;

use crate::helper_structs::{Animation, Direction, Font};

pub fn send_text(port: &mut dyn SerialPort, text_rows: &[String]) {
    for (i, row) in text_rows.iter().enumerate() {
        if row.is_empty() {
            continue;
        }
        let result = serialize_write_line(i as u8, row.as_str());
        if let Some(packet) = result {
            let _result = port.write(&packet);
            let mut buffer = [0; 20];
            let _result = port.read(&mut buffer);
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}

pub fn send_colors(port: &mut dyn SerialPort, color_rows: &[RgbColor]) {
    for (i, row) in color_rows.iter().enumerate() {
        let result = serialize_set_color(
            i as u8,
            RgbColor {
                r: row.r,
                g: row.g,
                b: row.b,
            },
        );
        if let Some(packet) = result {
            let _result = port.write(&packet);
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
                let speed = 60 / speed.unwrap();

                libuartmatrix::enums::Animation::BlinkAnimation(speed)
            }
            Animation::Slide => {
                let speed = str::parse::<u8>(speed);
                
                if speed.is_err() {
                    continue;
                }

                let speed = 60 / speed.unwrap();

                if direction.is_none() {
                    continue;
                }

                let dir = match direction.unwrap() {
                    Direction::Left => libuartmatrix::enums::Direction::Left,
                    Direction::Right => libuartmatrix::enums::Direction::Right,
                };

                libuartmatrix::enums::Animation::SlideAnimation(speed, dir)
            }
            Animation::None => libuartmatrix::enums::Animation::NoAnimation,
        };

        let result = serialize_set_animation(i as u8, animation);
        if let Some(packet) = result {
            let _result = port.write(&packet);
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
            let _result = port.write(&packet);
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}

pub fn send_change_mode(port: &mut dyn SerialPort, mode: DisplayMode) {
    let result = serialize_switch_mode(mode);
    if let Some(packet) = result {
        let _result = port.write(&packet);
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
            let _result = port.write(&packet);
            thread::sleep(time::Duration::from_millis(50));
        }
    }
}

pub fn send_draw_pixel(port: &mut dyn SerialPort, x: &str, y: &str, color: &RgbColor) {
    let x: Result<u8, _> = x.parse();
    let y: Result<u8, _> = y.parse();

    if x.is_err() || y.is_err() {
        return;
    }

    let result = serialize_draw_pixel(
        Point {
            x: x.unwrap(),
            y: y.unwrap(),
        },
        *color,
    );
    if let Some(packet) = result {
        let _result = port.write(&packet);
        thread::sleep(time::Duration::from_millis(50));
    }
}

pub fn send_draw_line(
    port: &mut dyn SerialPort,
    x_1: &str,
    y_1: &str,
    x_2: &str,
    y_2: &str,
    color: &RgbColor,
    thickness: &str,
) {
    let x_1: Result<u8, _> = x_1.parse();
    let y_1: Result<u8, _> = y_1.parse();
    let x_2: Result<u8, _> = x_2.parse();
    let y_2: Result<u8, _> = y_2.parse();
    let thickness: Result<u8, _> = thickness.parse();

    if x_1.is_err() || y_1.is_err() || x_2.is_err() || y_2.is_err() || thickness.is_err() {
        return;
    }

    let result = serialize_draw_line(
        Point {
            x: x_1.unwrap(),
            y: y_1.unwrap(),
        },
        Point {
            x: x_2.unwrap(),
            y: y_2.unwrap(),
        },
        thickness.unwrap(),
        *color,
    );

    if let Some(packet) = result {
        let _result = port.write(&packet);
        thread::sleep(time::Duration::from_millis(50));
    }
}

pub fn send_draw_rectangle(
    port: &mut dyn SerialPort,
    x_1: &str,
    y_1: &str,
    x_2: &str,
    y_2: &str,
    color: &RgbColor,
    thickness: &str,
    filled: bool,
) {
    let x_1: Result<u8, _> = x_1.parse();
    let y_1: Result<u8, _> = y_1.parse();
    let x_2: Result<u8, _> = x_2.parse();
    let y_2: Result<u8, _> = y_2.parse();
    let thickness: Result<u8, _> = thickness.parse();

    if x_1.is_err() || y_1.is_err() || x_2.is_err() || y_2.is_err() || thickness.is_err() {
        return;
    }

    let result = serialize_draw_rectangle(
        Point {
            x: x_1.unwrap(),
            y: y_1.unwrap(),
        },
        Point {
            x: x_2.unwrap(),
            y: y_2.unwrap(),
        },
        thickness.unwrap(),
        *color,
        filled,
    );

    if let Some(packet) = result {
        let _result = port.write(&packet);
        thread::sleep(time::Duration::from_millis(50));
    }
}

pub fn send_draw_triangle(
    port: &mut dyn SerialPort,
    x_1: &str,
    y_1: &str,
    x_2: &str,
    y_2: &str,
    x_3: &str,
    y_3: &str,
    color: &RgbColor,
    thickness: &str,
    filled: bool,
) {
    let x_1: Result<u8, _> = x_1.parse();
    let y_1: Result<u8, _> = y_1.parse();
    let x_2: Result<u8, _> = x_2.parse();
    let y_2: Result<u8, _> = y_2.parse();
    let x_3: Result<u8, _> = x_3.parse();
    let y_3: Result<u8, _> = y_3.parse();
    let thickness: Result<u8, _> = thickness.parse();

    if x_1.is_err()
        || y_1.is_err()
        || x_2.is_err()
        || y_2.is_err()
        || x_3.is_err()
        || y_3.is_err()
        || thickness.is_err()
    {
        return;
    }

    let result = serialize_draw_triangle(
        Point {
            x: x_1.unwrap(),
            y: y_1.unwrap(),
        },
        Point {
            x: x_2.unwrap(),
            y: y_2.unwrap(),
        },
        Point {
            x: x_3.unwrap(),
            y: y_3.unwrap(),
        },
        thickness.unwrap(),
        *color,
        filled,
    );

    if let Some(packet) = result {
        let _result = port.write(&packet);
        thread::sleep(time::Duration::from_millis(50));
    }
}

pub fn send_draw_circle(
    port: &mut dyn SerialPort,
    x_1: &str,
    y_1: &str,
    radius: &str,
    color: &RgbColor,
    thickness: &str,
    filled: bool,
) {
    let x_1: Result<u8, _> = x_1.parse();
    let y_1: Result<u8, _> = y_1.parse();
    let radius: Result<u8, _> = radius.parse();
    let thickness: Result<u8, _> = thickness.parse();

    if x_1.is_err() || y_1.is_err() || radius.is_err() || thickness.is_err() {
        return;
    }

    let result = serialize_draw_circle(
        Point {
            x: x_1.unwrap(),
            y: y_1.unwrap(),
        },
        radius.unwrap(),
        thickness.unwrap(),
        *color,
        filled,
    );

    if let Some(packet) = result {
        let _result = port.write(&packet);
        thread::sleep(time::Duration::from_millis(50));
    }
}

pub fn send_clear_screen(port: &mut dyn SerialPort) {
    let result = serialize_clear();

    if let Some(packet) = result {
        let _result = port.write(&packet);
        thread::sleep(time::Duration::from_millis(50));
    }
}
