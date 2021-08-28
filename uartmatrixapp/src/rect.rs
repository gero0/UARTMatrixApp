use iced::{
    widget::canvas::{Cursor, Frame, Geometry, Path, Program, Stroke},
    Color, Rectangle, Size,
};

use crate::{helper_structs::RgbColor, Message};

#[derive(Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub color: RgbColor,
}

impl Program<Message> for Rect {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        // We prepare a new `Frame`
        let mut frame = Frame::new(bounds.size());

        let rect = Path::rectangle(
            frame.center(),
            Size {
                width: self.x,
                height: self.y,
            },
        );

        frame.fill(
            &rect,
            Color::from_rgb(
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
            ),
        );

        frame.stroke(
            &rect,
            Stroke {
                color: Color::BLACK,
                width: 1.0,
                line_cap: Default::default(),
                line_join: Default::default(),
            },
        );

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}
