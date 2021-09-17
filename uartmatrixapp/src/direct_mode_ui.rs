use iced::{Button, Canvas, Checkbox, Column, Length, Row, Slider, Text, TextInput};

use libuartmatrix::RgbColor;

use crate::rect::Rect;
use crate::text_mode_ui::COLOR_SLIDER_RANGE;
use crate::Message;

pub struct DirectModeData {
    pub draw_pixel_btn: iced::button::State,
    pub draw_line_btn: iced::button::State,
    pub draw_rectangle_btn: iced::button::State,
    pub draw_triangle_btn: iced::button::State,
    pub draw_circle_btn: iced::button::State,
    pub cancel_drawing_btn: iced::button::State,
    pub load_file_btn: iced::button::State,

    pub color_slider_states: [iced::slider::State; 3],
    pub color_slider_values: RgbColor,

    pub pixel_x_text_state: iced::text_input::State,
    pub pixel_y_text_state: iced::text_input::State,

    pub line_x_1_text_state: iced::text_input::State,
    pub line_y_1_text_state: iced::text_input::State,
    pub line_x_2_text_state: iced::text_input::State,
    pub line_y_2_text_state: iced::text_input::State,

    pub rectangle_x_1_text_state: iced::text_input::State,
    pub rectangle_y_1_text_state: iced::text_input::State,
    pub rectangle_x_2_text_state: iced::text_input::State,
    pub rectangle_y_2_text_state: iced::text_input::State,

    pub triangle_x_1_text_state: iced::text_input::State,
    pub triangle_y_1_text_state: iced::text_input::State,
    pub triangle_x_2_text_state: iced::text_input::State,
    pub triangle_y_2_text_state: iced::text_input::State,
    pub triangle_x_3_text_state: iced::text_input::State,
    pub triangle_y_3_text_state: iced::text_input::State,

    pub circle_x_text_state: iced::text_input::State,
    pub circle_y_text_state: iced::text_input::State,
    pub circle_radius_text_state: iced::text_input::State,

    pub pixel_x_text_input: String,
    pub pixel_y_text_input: String,

    pub line_x_1_text_input: String,
    pub line_y_1_text_input: String,
    pub line_x_2_text_input: String,
    pub line_y_2_text_input: String,

    pub rectangle_x_1_text_input: String,
    pub rectangle_y_1_text_input: String,
    pub rectangle_x_2_text_input: String,
    pub rectangle_y_2_text_input: String,

    pub triangle_x_1_text_input: String,
    pub triangle_y_1_text_input: String,
    pub triangle_x_2_text_input: String,
    pub triangle_y_2_text_input: String,
    pub triangle_x_3_text_input: String,
    pub triangle_y_3_text_input: String,

    pub circle_x_text_input: String,
    pub circle_y_text_input: String,
    pub circle_radius_text_input: String,

    pub filled_value: bool,
    pub shape_thickness_state: iced::text_input::State,
    pub shape_thickness_value: String,
    pub clear_screen_btn: iced::button::State,
}

impl DirectModeData {
    pub fn new() -> Self {
        DirectModeData {
            draw_pixel_btn: iced::button::State::new(),
            draw_line_btn: iced::button::State::new(),
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

            pixel_x_text_state: iced::text_input::State::new(),
            pixel_y_text_state: iced::text_input::State::new(),

            line_x_1_text_state: iced::text_input::State::new(),
            line_y_1_text_state: iced::text_input::State::new(),
            line_x_2_text_state: iced::text_input::State::new(),
            line_y_2_text_state: iced::text_input::State::new(),

            rectangle_x_1_text_state: iced::text_input::State::new(),
            rectangle_y_1_text_state: iced::text_input::State::new(),
            rectangle_x_2_text_state: iced::text_input::State::new(),
            rectangle_y_2_text_state: iced::text_input::State::new(),

            triangle_x_1_text_state: iced::text_input::State::new(),
            triangle_y_1_text_state: iced::text_input::State::new(),
            triangle_x_2_text_state: iced::text_input::State::new(),
            triangle_y_2_text_state: iced::text_input::State::new(),
            triangle_x_3_text_state: iced::text_input::State::new(),
            triangle_y_3_text_state: iced::text_input::State::new(),

            circle_x_text_state: iced::text_input::State::new(),
            circle_y_text_state: iced::text_input::State::new(),
            circle_radius_text_state: iced::text_input::State::new(),

            pixel_x_text_input: String::from(""),
            pixel_y_text_input: String::from(""),

            line_x_1_text_input: String::from(""),
            line_y_1_text_input: String::from(""),
            line_x_2_text_input: String::from(""),
            line_y_2_text_input: String::from(""),

            rectangle_x_1_text_input: String::from(""),
            rectangle_y_1_text_input: String::from(""),
            rectangle_x_2_text_input: String::from(""),
            rectangle_y_2_text_input: String::from(""),

            triangle_x_1_text_input: String::from(""),
            triangle_y_1_text_input: String::from(""),
            triangle_x_2_text_input: String::from(""),
            triangle_y_2_text_input: String::from(""),
            triangle_x_3_text_input: String::from(""),
            triangle_y_3_text_input: String::from(""),

            circle_x_text_input: String::from(""),
            circle_y_text_input: String::from(""),
            circle_radius_text_input: String::from(""),

            filled_value: false,
            shape_thickness_state: iced::text_input::State::new(),
            shape_thickness_value: String::from(""),

            clear_screen_btn: iced::button::State::new(),

            color_slider_values: RgbColor::new(),
        }
    }
}

pub fn add_direct_mode_ui(data: &mut DirectModeData) -> (Column<Message>, Column<Message>) {
    let mut left_column = Column::new().max_width(800).spacing(20);
    let mut right_column = Column::new().max_width(600).spacing(20);

    let load_file_button = Button::new(&mut data.load_file_btn, Text::new("Load image..."))
        .on_press(Message::LoadImage);

    let clear_screen_button = Button::new(&mut data.clear_screen_btn, Text::new("Clear screen"))
        .on_press(Message::ClearScreen);

    let thickness_field = TextInput::new(
        &mut data.shape_thickness_state,
        "",
        &data.shape_thickness_value,
        Message::ThicknessChanged,
    );
    let filled_checkbox = Checkbox::new(data.filled_value, "Filled", Message::FilledChanged);
    let common_row = Row::new()
        .spacing(20)
        .push(Text::new("Thickness:"))
        .push(thickness_field)
        .push(filled_checkbox);

    let pixel_x_text_input = TextInput::new(
        &mut data.pixel_x_text_state,
        "",
        &data.pixel_x_text_input,
        move |value| Message::PixelCoordChanged(value, 0),
    );

    let pixel_y_text_input = TextInput::new(
        &mut data.pixel_y_text_state,
        "",
        &data.pixel_y_text_input,
        move |value| Message::PixelCoordChanged(value, 1),
    );

    let draw_pixel_button =
        Button::new(&mut data.draw_pixel_btn, Text::new("Draw Pixel")).on_press(Message::DrawPixel);

    let pixel_row = Row::new()
        .spacing(20)
        .push(Text::new("x:"))
        .push(pixel_x_text_input)
        .push(Text::new("y:"))
        .push(pixel_y_text_input)
        .push(draw_pixel_button);

    let line_x_1_text_input = TextInput::new(
        &mut data.line_x_1_text_state,
        "",
        &data.line_x_1_text_input,
        move |value| Message::LineCoordChanged(value, 0, 0),
    );

    let line_y_1_text_input = TextInput::new(
        &mut data.line_y_1_text_state,
        "",
        &data.line_y_1_text_input,
        move |value| Message::LineCoordChanged(value, 0, 1),
    );

    let line_x_2_text_input = TextInput::new(
        &mut data.line_x_2_text_state,
        "",
        &data.line_x_2_text_input,
        move |value| Message::LineCoordChanged(value, 1, 0),
    );

    let line_y_2_text_input = TextInput::new(
        &mut data.line_y_2_text_state,
        "",
        &data.line_y_2_text_input,
        move |value| Message::LineCoordChanged(value, 1, 1),
    );

    let draw_line_button =
        Button::new(&mut data.draw_line_btn, Text::new("Draw Line")).on_press(Message::DrawLine);

    let line_row = Row::new()
        .spacing(20)
        .push(Text::new("x1:"))
        .push(line_x_1_text_input)
        .push(Text::new("y1:"))
        .push(line_y_1_text_input)
        .push(Text::new("x2:"))
        .push(line_x_2_text_input)
        .push(Text::new("y2:"))
        .push(line_y_2_text_input)
        .push(draw_line_button);

    let rectangle_x_1_text_input = TextInput::new(
        &mut data.rectangle_x_1_text_state,
        "",
        &data.rectangle_x_1_text_input,
        move |value| Message::RectangleCoordChanged(value, 0, 0),
    );

    let rectangle_y_1_text_input = TextInput::new(
        &mut data.rectangle_y_1_text_state,
        "",
        &data.rectangle_y_1_text_input,
        move |value| Message::RectangleCoordChanged(value, 0, 1),
    );

    let rectangle_x_2_text_input = TextInput::new(
        &mut data.rectangle_x_2_text_state,
        "",
        &data.rectangle_x_2_text_input,
        move |value| Message::RectangleCoordChanged(value, 1, 0),
    );

    let rectangle_y_2_text_input = TextInput::new(
        &mut data.rectangle_y_2_text_state,
        "",
        &data.rectangle_y_2_text_input,
        move |value| Message::RectangleCoordChanged(value, 1, 1),
    );

    let draw_rectangle_button =
        Button::new(&mut data.draw_rectangle_btn, Text::new("Draw Rectangle"))
            .on_press(Message::DrawRectangle);

    let rectangle_row = Row::new()
        .spacing(20)
        .push(Text::new("x1:"))
        .push(rectangle_x_1_text_input)
        .push(Text::new("y1:"))
        .push(rectangle_y_1_text_input)
        .push(Text::new("x2:"))
        .push(rectangle_x_2_text_input)
        .push(Text::new("y2:"))
        .push(rectangle_y_2_text_input)
        .push(draw_rectangle_button);

    let triangle_x_1_text_input = TextInput::new(
        &mut data.triangle_x_1_text_state,
        "",
        &data.triangle_x_1_text_input,
        move |value| Message::TriangleCoordChanged(value, 0, 0),
    );

    let triangle_y_1_text_input = TextInput::new(
        &mut data.triangle_y_1_text_state,
        "",
        &data.triangle_y_1_text_input,
        move |value| Message::TriangleCoordChanged(value, 0, 1),
    );

    let triangle_x_2_text_input = TextInput::new(
        &mut data.triangle_x_2_text_state,
        "",
        &data.triangle_x_2_text_input,
        move |value| Message::TriangleCoordChanged(value, 1, 0),
    );

    let triangle_y_2_text_input = TextInput::new(
        &mut data.triangle_y_2_text_state,
        "",
        &data.triangle_y_2_text_input,
        move |value| Message::TriangleCoordChanged(value, 1, 1),
    );

    let triangle_x_3_text_input = TextInput::new(
        &mut data.triangle_x_3_text_state,
        "",
        &data.triangle_x_3_text_input,
        move |value| Message::TriangleCoordChanged(value, 2, 0),
    );

    let triangle_y_3_text_input = TextInput::new(
        &mut data.triangle_y_3_text_state,
        "",
        &data.triangle_y_3_text_input,
        move |value| Message::TriangleCoordChanged(value, 2, 1),
    );

    let draw_triangle_button = Button::new(&mut data.draw_triangle_btn, Text::new("Draw Triangle"))
        .on_press(Message::DrawTriangle);

    let triangle_row = Row::new()
        .spacing(20)
        .push(Text::new("x1:"))
        .push(triangle_x_1_text_input)
        .push(Text::new("y1:"))
        .push(triangle_y_1_text_input)
        .push(Text::new("x2:"))
        .push(triangle_x_2_text_input)
        .push(Text::new("y2:"))
        .push(triangle_y_2_text_input)
        .push(Text::new("x3:"))
        .push(triangle_x_3_text_input)
        .push(Text::new("y3:"))
        .push(triangle_y_3_text_input)
        .push(draw_triangle_button);

    let circle_x_text_input = TextInput::new(
        &mut data.circle_x_text_state,
        "",
        &data.circle_x_text_input,
        move |value| Message::CircleCoordChanged(value, 0),
    );

    let circle_y_text_input = TextInput::new(
        &mut data.circle_y_text_state,
        "",
        &data.circle_y_text_input,
        move |value| Message::CircleCoordChanged(value, 1),
    );

    let circle_radius_text_input = TextInput::new(
        &mut data.circle_radius_text_state,
        "",
        &data.circle_radius_text_input,
        move |value| Message::CircleCoordChanged(value, 2),
    );

    let draw_circle_button = Button::new(&mut data.draw_circle_btn, Text::new("Draw Circle"))
        .on_press(Message::DrawCircle);

    let circle_row = Row::new()
        .spacing(20)
        .push(Text::new("x:"))
        .push(circle_x_text_input)
        .push(Text::new("y:"))
        .push(circle_y_text_input)
        .push(Text::new("radius:"))
        .push(circle_radius_text_input)
        .push(draw_circle_button);

    left_column = left_column
        .push(load_file_button)
        .push(clear_screen_button)
        .push(Text::new("Draw shape"));

    left_column = add_slider_section(
        left_column,
        &mut data.color_slider_states,
        &mut data.color_slider_values,
    );

    left_column = left_column
        .push(common_row)
        .push(pixel_row)
        .push(line_row)
        .push(rectangle_row)
        .push(triangle_row)
        .push(circle_row);

    (left_column, right_column)
}

fn add_slider_section<'a>(
    mut column: Column<'a, Message>,
    states: &'a mut [iced::slider::State; 3],
    values: &'a mut RgbColor,
) -> Column<'a, Message> {
    let [state_r, state_g, state_b] = states;

    let slider_r = Slider::new(state_r, COLOR_SLIDER_RANGE, values.r as i32, move |value| {
        Message::ShapeColorChanged(value, 0)
    });
    let slider_g = Slider::new(state_g, COLOR_SLIDER_RANGE, values.g as i32, move |value| {
        Message::ShapeColorChanged(value, 1)
    });
    let slider_b = Slider::new(state_b, COLOR_SLIDER_RANGE, values.b as i32, move |value| {
        Message::ShapeColorChanged(value, 2)
    });

    let color_preview = Canvas::new(Rect {
        x: -12.0,
        y: -12.0,
        color: *values,
    })
    .width(Length::Units(25))
    .height(Length::Units(25));

    let slider_row = Row::new()
        .spacing(20)
        .push(Text::new("r"))
        .push(slider_r)
        .push(Text::new(values.r.to_string()))
        .push(Text::new("g"))
        .push(slider_g)
        .push(Text::new(values.g.to_string()))
        .push(Text::new("b"))
        .push(slider_b)
        .push(Text::new(values.b.to_string()))
        .push(color_preview);

    let caption = Text::new("Color");

    column = column.push(caption).push(slider_row);

    column
}
