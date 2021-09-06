use crate::enums::{Animation, DisplayMode, FontType};

pub mod enums;

pub const MAX_FRAME_SIZE: usize = 512;
pub const MAX_TEXT_LENGTH: usize = 255;

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

    for i in 0..content.len() {
        buffer[i + 5] = content[i];
    }

    return Some(buffer);
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

pub fn serialize_set_color(row: u8, r: u8, g: u8, b: u8) -> Option<[u8; MAX_FRAME_SIZE]> {
    let data = [4, row, r, g, b];
    serialize_umx_frame(&data)
}

pub fn serialize_set_animation(row: u8, animation: Animation) -> Option<[u8; MAX_FRAME_SIZE]> {
    let mut anim_data: Vec<u8> = animation.into();
    let mut data = vec![5, row];
    data.append(&mut anim_data);

    serialize_umx_frame(data.as_slice())
}

pub fn serialize_draw_pixel(x: u8, y: u8, r: u8, g: u8, b: u8) -> Option<[u8; MAX_FRAME_SIZE]> {
    let data = [6, x, y, r, g, b];
    serialize_umx_frame(&data)
}

pub fn serialize_draw_row(row: u8, pixels: Vec<(u8, u8, u8)>) -> Option<[u8; MAX_FRAME_SIZE]> {
    let mut data = vec![];
    data.push(7);
    data.push(row);
    for pixel in pixels {
        data.push(pixel.0);
        data.push(pixel.1);
        data.push(pixel.2);
    }

    serialize_umx_frame(data.as_slice())
}

pub fn serialize_clear() -> Option<[u8; MAX_FRAME_SIZE]> {
    serialize_umx_frame(&[8])
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

    fn serialize_write_line_test() {
        let frame = serialize_write_line(3, "THISISATEST").unwrap();
        assert_eq!(frame[5], 2);
        assert_eq!(frame[6], 3);
        assert_eq!(&from_utf8(&frame[7..13]).unwrap()[0..12], "THISISATEST");
    }
}
