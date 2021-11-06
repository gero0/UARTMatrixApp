use crate::enums::{Animation, Direction, DisplayMode};

use std::os::raw::*;

pub mod enums;
mod crc;

pub const MAX_FRAME_SIZE: usize = 512;
pub const MAX_TEXT_LENGTH: usize = 255;

#[repr(C)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new() -> Self {
        RgbColor {
            r: 128,
            g: 128,
            b: 128,
        }
    }
}

fn serialize_umx_frame(content: &[u8]) -> Option<[u8; MAX_FRAME_SIZE]> {
    if content.len() > MAX_FRAME_SIZE {
        return None;
    }
    let mut buffer = [0; MAX_FRAME_SIZE];
    buffer[0] = b'U';
    buffer[1] = b'M';
    buffer[2] = b'X';
    let length = content.len() as u16;
    buffer[3] = (length >> 8) as u8;
    buffer[4] = length as u8;

    buffer[5..(content.len() + 5)].clone_from_slice(&content);
    buffer[content.len() + 5] = crc::crc8_ccitt(&buffer[5..(content.len() + 5)]);

    Some(buffer)
}

pub fn serialize_param_request() -> Option<[u8; MAX_FRAME_SIZE]> {
    serialize_umx_frame(&[0])
}

pub fn serialize_switch_mode(mode: DisplayMode) -> Option<[u8; MAX_FRAME_SIZE]> {
    serialize_umx_frame(&[1, mode.into()])
}

pub fn serialize_write_line(row: u8, text: &str) -> Option<[u8; MAX_FRAME_SIZE]> {
    if text.len() > MAX_TEXT_LENGTH {
        return None;
    }

    let data = [&[2], &[row], text.as_bytes()].concat();

    serialize_umx_frame(data.as_slice())
}

pub fn serialize_set_font(row: u8, font: u8) -> Option<[u8; MAX_FRAME_SIZE]> {
    let data = [3, row, font];
    serialize_umx_frame(&data)
}

pub fn serialize_set_color(row: u8, color: RgbColor) -> Option<[u8; MAX_FRAME_SIZE]> {
    let RgbColor { r, g, b } = color;
    let data = [4, row, r, g, b];
    serialize_umx_frame(&data)
}

pub fn serialize_set_animation(row: u8, animation: Animation) -> Option<[u8; MAX_FRAME_SIZE]> {
    let mut anim_data: Vec<u8> = animation.into();
    let mut data = vec![5, row];
    data.append(&mut anim_data);

    serialize_umx_frame(data.as_slice())
}

pub fn serialize_draw_pixel(position: Point, color: RgbColor) -> Option<[u8; MAX_FRAME_SIZE]> {
    let Point { x, y } = position;
    let RgbColor { r, g, b } = color;
    let data = [6, x, y, r, g, b];
    serialize_umx_frame(&data)
}

pub fn serialize_draw_line(
    point_1: Point,
    point_2: Point,
    thickness: u8,
    color: RgbColor,
) -> Option<[u8; MAX_FRAME_SIZE]> {
    let Point { x: x_1, y: y_1 } = point_1;
    let Point { x: x_2, y: y_2 } = point_2;
    let RgbColor { r, g, b } = color;

    let data = [8, x_1, y_1, x_2, y_2, thickness, r, g, b];

    serialize_umx_frame(&data)
}

pub fn serialize_draw_rectangle(
    point_1: Point,
    point_2: Point,
    thickness: u8,
    color: RgbColor,
    filled: bool,
) -> Option<[u8; MAX_FRAME_SIZE]> {
    let Point { x: x_1, y: y_1 } = point_1;
    let Point { x: x_2, y: y_2 } = point_2;
    let RgbColor { r, g, b } = color;

    let filled = if filled { 1 } else { 0 };

    let data = [9, x_1, y_1, x_2, y_2, thickness, r, g, b, filled];

    serialize_umx_frame(&data)
}

pub fn serialize_draw_triangle(
    point_1: Point,
    point_2: Point,
    point_3: Point,
    thickness: u8,
    color: RgbColor,
    filled: bool,
) -> Option<[u8; MAX_FRAME_SIZE]> {
    let Point { x: x_1, y: y_1 } = point_1;
    let Point { x: x_2, y: y_2 } = point_2;
    let Point { x: x_3, y: y_3 } = point_3;
    let RgbColor { r, g, b } = color;

    let filled = if filled { 1 } else { 0 };

    let data = [10, x_1, y_1, x_2, y_2, x_3, y_3, thickness, r, g, b, filled];

    serialize_umx_frame(&data)
}

pub fn serialize_draw_circle(
    center: Point,
    radius: u8,
    thickness: u8,
    color: RgbColor,
    filled: bool,
) -> Option<[u8; MAX_FRAME_SIZE]> {
    let Point { x, y } = center;
    let RgbColor { r, g, b } = color;

    let filled = if filled { 1 } else { 0 };

    let data = [11, x, y, radius, thickness, r, g, b, filled];

    serialize_umx_frame(&data)
}

pub fn serialize_draw_row(row: u8, pixels: Vec<(u8, u8, u8)>) -> Option<[u8; MAX_FRAME_SIZE]> {
    let mut data = vec![7, row];
    for pixel in pixels {
        data.push(pixel.0);
        data.push(pixel.1);
        data.push(pixel.2);
    }

    serialize_umx_frame(data.as_slice())
}

pub fn serialize_clear() -> Option<[u8; MAX_FRAME_SIZE]> {
    serialize_umx_frame(&[12])
}

pub fn serialize_enable_output() -> Option<[u8; MAX_FRAME_SIZE]> {
    serialize_umx_frame(&[9])
}

pub fn serialize_disable_output() -> Option<[u8; MAX_FRAME_SIZE]> {
    serialize_umx_frame(&[10])
}

pub fn serialize_ping() -> Option<[u8; MAX_FRAME_SIZE]> {
    serialize_umx_frame(&[11])
}

//FFI interfaces

fn opt_array_to_ffi(buffer: *mut c_uchar, array: Option<[u8; MAX_FRAME_SIZE]>) -> c_int {
    match array {
        Some(array) => {
            unsafe {
                std::ptr::copy(array.as_ptr(), buffer as *mut u8, array.len());
            }
            return array.len() as c_int;
        }
        None => return -1 as c_int,
    }
}

#[no_mangle]
pub extern "C" fn umx_serialize_param_request(buffer: *mut c_uchar) -> c_int {
    let result = serialize_param_request();
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_switch_mode(buffer: *mut c_uchar, mode: c_int) -> c_int {
    let mode = match mode {
        0 => DisplayMode::Text,
        _ => DisplayMode::Direct,
    };

    let result = serialize_switch_mode(mode);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_write_line(
    buffer: *mut c_uchar,
    row: c_uchar,
    text: *const c_uchar,
    text_len: c_uint,
) -> c_int {
    unsafe {
        let slice = core::slice::from_raw_parts(text, text_len as usize);
        let string = core::str::from_utf8(slice);
        match string {
            Ok(string) => {
                let result = serialize_write_line(row as u8, string);
                return opt_array_to_ffi(buffer, result);
            }
            Err(_e) => {
                return -1;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn umx_serialize_set_font(
    buffer: *mut c_uchar,
    row: c_uchar,
    font: c_uchar,
) -> c_int {
    let result = serialize_set_font(row, font);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_set_color(
    buffer: *mut c_uchar,
    row: c_uchar,
    color: RgbColor,
) -> c_int {
    let result = serialize_set_color(row, color);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_set_animation(
    buffer: *mut c_uchar,
    row: c_uchar,
    animation: c_uchar,
    speed: c_uchar,
    direction: c_uchar,
) -> c_int {
    let animation = match animation {
        1 => Animation::BlinkAnimation(speed),
        2 => {
            let dir = match direction {
                0 => Direction::Left,
                _ => Direction::Right,
            };
            Animation::SlideAnimation(speed, dir)
        }
        _ => Animation::NoAnimation,
    };
    let result = serialize_set_animation(row, animation);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_draw_pixel(
    buffer: *mut c_uchar,
    position: Point,
    color: RgbColor,
) -> c_int {
    let result = serialize_draw_pixel(position, color);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_draw_rectangle(
    buffer: *mut c_uchar,
    point_1: Point,
    point_2: Point,
    thickness: c_uchar,
    color: RgbColor,
    filled: c_int,
) -> c_int {
    let filled = match filled {
        0 => false,
        _ => true,
    };
    let result = serialize_draw_rectangle(point_1, point_2, thickness as u8, color, filled);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_draw_triangle(
    buffer: *mut c_uchar,
    point_1: Point,
    point_2: Point,
    point_3: Point,
    thickness: c_uchar,
    color: RgbColor,
    filled: c_int,
) -> c_int {
    let filled = match filled {
        0 => false,
        _ => true,
    };
    let result = serialize_draw_triangle(point_1, point_2, point_3, thickness as u8, color, filled);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_draw_circle(
    buffer: *mut c_uchar,
    center: Point,
    radius: c_uchar,
    thickness: c_uchar,
    color: RgbColor,
    filled: c_int,
) -> c_int {
    let filled = match filled {
        0 => false,
        _ => true,
    };
    let result = serialize_draw_circle(center, radius as u8, thickness as u8, color, filled);
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_draw_row(
    buffer: *mut c_uchar,
    row: c_uchar,
    pixels: *const RgbColor,
    pixels_len: c_uint,
) -> c_int{
    unsafe {
        let mut vec = vec![];
        let slice = core::slice::from_raw_parts(pixels, pixels_len as usize);
        slice
            .iter()
            .for_each(|element| vec.push((element.r, element.g, element.b)));

        let result = serialize_draw_row(row, vec);
        return opt_array_to_ffi(buffer, result);
    }
}

#[no_mangle]
pub extern "C" fn umx_serialize_clear(buffer: *mut c_uchar) -> c_int {
    let result = serialize_clear();
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_enable_output(buffer: *mut c_uchar) -> c_int {
    let result = serialize_enable_output();
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_disable_output(buffer: *mut c_uchar) -> c_int {
    let result = serialize_disable_output();
    return opt_array_to_ffi(buffer, result);
}

#[no_mangle]
pub extern "C" fn umx_serialize_ping(buffer: *mut c_uchar) -> c_int {
    let result = serialize_ping();
    return opt_array_to_ffi(buffer, result);
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn serialize_umx_test() {
        let frame = serialize_umx_frame(&[27]).unwrap();
        // [85, 77, 88, l]
        assert_eq!(frame[0], 85);
        assert_eq!(frame[1], 77);
        assert_eq!(frame[2], 88);
        assert_eq!(frame[3], 1);
        assert_eq!(frame[4], 27);
    }

    #[test]
    fn serialize_write_line_test() {
        let frame = serialize_write_line(3, "THISISATEST").unwrap();
        assert_eq!(frame[5], 2);
        assert_eq!(frame[6], 3);
        assert_eq!(&from_utf8(&frame[7..13]).unwrap()[0..12], "THISISATEST");
    }
}
