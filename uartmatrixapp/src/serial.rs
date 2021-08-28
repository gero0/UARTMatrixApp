use std::io::{Read, Write};
use std::{thread, time};

use serialport::SerialPort;

use libuartmatrix::*;

use crate::helper_structs::RgbColor;

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
