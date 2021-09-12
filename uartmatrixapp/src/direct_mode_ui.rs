use iced::{Button, Column, Row, Text};

use crate::Message;

pub enum DrawingState {
    None,
    Pixel(i32, i32),
    Rectangle([(i32, i32); 4]),
    Circle((i32, i32, f32)),
}

pub struct DirectModeData {
    pub draw_pixel_btn: iced::button::State,
    pub draw_rectangle_btn: iced::button::State,
    pub draw_triangle_btn: iced::button::State,
    pub draw_circle_btn: iced::button::State,
    pub cancel_drawing_btn: iced::button::State,
    pub load_file_btn: iced::button::State,

    pub color_slider_states: [iced::slider::State; 3],

    pub drawing_state: DrawingState,
}

impl DirectModeData {
    pub fn new() -> Self {
        DirectModeData {
            draw_pixel_btn: iced::button::State::new(),
            draw_rectangle_btn: iced::button::State::new(),
            draw_triangle_btn: iced::button::State::new(),
            draw_circle_btn: iced::button::State::new(),
            cancel_drawing_btn: iced::button::State::new(),
            load_file_btn: iced::button::State::new(),

            color_slider_states: [
                iced::slider::State::new(),
                iced::slider::State::new(),
                iced::slider::State::new(),
            ],
            drawing_state: DrawingState::None,
        }
    }
}

pub fn add_direct_mode_ui(data: &mut DirectModeData) -> (Column<Message>, Column<Message>) {
    let mut left_column = Column::new().max_width(600).spacing(20);
    let mut right_column = Column::new().max_width(600).spacing(20);

    let load_file_button = Button::new(&mut data.load_file_btn, Text::new("Load image..."))
        .on_press(Message::LoadImage);

    let mut button_row = Row::new().spacing(20);

    let draw_pixel_button =
        Button::new(&mut data.draw_pixel_btn, Text::new("Draw Pixel")).on_press(Message::DrawPixel);

    let draw_rectangle_button =
        Button::new(&mut data.draw_rectangle_btn, Text::new("Draw Rectangle"))
            .on_press(Message::DrawRectangle);

    let draw_triangle_button = Button::new(&mut data.draw_triangle_btn, Text::new("Draw Triangle"))
        .on_press(Message::DrawTriangle);

    let draw_circle_button = Button::new(&mut data.draw_circle_btn, Text::new("Draw Circle"))
        .on_press(Message::DrawCircle);

    button_row = button_row
        .push(draw_pixel_button)
        .push(draw_rectangle_button)
        .push(draw_triangle_button)
        .push(draw_circle_button);

    left_column = left_column.push(load_file_button).push(button_row);

    (left_column, right_column)
}
