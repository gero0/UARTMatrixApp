use std::ops::RangeInclusive;

use iced::{Button, Canvas, Column, Length, PickList, Row, Slider, Text, TextInput};

use libuartmatrix::RgbColor;

use crate::{
    helper_structs::{Animation, Direction, Font, RgbSlidersState},
    rect::Rect,
    Message,
};

pub const COLOR_SLIDER_RANGE: RangeInclusive<i32> = 0..=255;

pub struct TextModeData {
    pub send_colors_btn: iced::button::State,
    pub send_anims_btn: iced::button::State,
    pub send_text_btn: iced::button::State,
    pub send_fonts_btn: iced::button::State,
    pub anim_select_states: [iced::pick_list::State<Animation>; 3],
    pub anim_select_values: [Option<Animation>; 3],

    pub anim_speed_states: [iced::text_input::State; 3],
    pub anim_speed_values: [String; 3],

    pub anim_direction_states: [iced::pick_list::State<Direction>; 3],
    pub anim_direction_values: [Option<Direction>; 3],

    pub font_states: [iced::pick_list::State<Font>; 3],
    pub font_values: [Option<Font>; 3],

    pub text_rows_states: [iced::text_input::State; 3],
    pub text_rows_values: [String; 3],

    pub color_slider_states: [RgbSlidersState; 3],
    pub color_slider_values: [RgbColor; 3],
}

impl TextModeData {
    pub fn new() -> Self {
        TextModeData {
            send_colors_btn: iced::button::State::new(),
            send_text_btn: iced::button::State::new(),
            send_anims_btn: iced::button::State::new(),
            send_fonts_btn: iced::button::State::new(),

            anim_select_states: [
                iced::pick_list::State::default(),
                iced::pick_list::State::default(),
                iced::pick_list::State::default(),
            ],
            anim_select_values: [
                Some(Animation::None),
                Some(Animation::None),
                Some(Animation::None),
            ],

            anim_speed_states: [
                iced::text_input::State::new(),
                iced::text_input::State::new(),
                iced::text_input::State::new(),
            ],
            anim_speed_values: [String::from(""), String::from(""), String::from("")],

            anim_direction_states: [
                iced::pick_list::State::default(),
                iced::pick_list::State::default(),
                iced::pick_list::State::default(),
            ],
            anim_direction_values: [Some(Direction::Left); 3],

            font_states: [
                iced::pick_list::State::default(),
                iced::pick_list::State::default(),
                iced::pick_list::State::default(),
            ],
            font_values: [Some(Font::Default); 3],

            text_rows_states: [
                iced::text_input::State::new(),
                iced::text_input::State::new(),
                iced::text_input::State::new(),
            ],

            text_rows_values: [String::from(""), String::from(""), String::from("")],

            color_slider_states: [RgbSlidersState::new(); 3],
            color_slider_values: [RgbColor::new(); 3],
        }
    }
}

pub fn add_text_mode_ui(data: &mut TextModeData) -> (Column<Message>, Column<Message>) {
    let mut left_column = Column::new().max_width(600).spacing(20);
    let mut right_column = Column::new().max_width(600).spacing(20);

    left_column = add_text_section(
        left_column,
        &mut data.text_rows_states,
        &mut data.text_rows_values,
    );

    let send_text_button =
        Button::new(&mut data.send_text_btn, Text::new("Send text")).on_press(Message::SendText);

    left_column = left_column.push(send_text_button);

    left_column = add_slider_section(
        left_column,
        &mut data.color_slider_states,
        &mut data.color_slider_values,
    );

    let send_colors_button = Button::new(&mut data.send_colors_btn, Text::new("Set colors"))
        .on_press(Message::SendColors);

    left_column = left_column.push(send_colors_button);

    right_column = add_anim_section(
        right_column,
        &mut data.anim_select_states,
        &mut data.anim_select_values,
        &mut data.anim_speed_states,
        &mut data.anim_speed_values,
        &mut data.anim_direction_states,
        &mut data.anim_direction_values,
    );

    let send_anims_button = Button::new(&mut data.send_anims_btn, Text::new("Set animations"))
        .on_press(Message::SendAnims);

    right_column = right_column.push(send_anims_button);

    right_column = add_font_section(right_column, &mut data.font_states, &mut data.font_values);

    let send_fonts_button =
        Button::new(&mut data.send_fonts_btn, Text::new("Set fonts")).on_press(Message::SendFonts);

    right_column = right_column.push(send_fonts_button);

    (left_column, right_column)
}

fn add_text_section<'a>(
    mut content: Column<'a, Message>,
    states: &'a mut [iced::text_input::State; 3],
    strings: &'a mut [String; 3],
) -> Column<'a, Message> {
    content = content.push(Text::new("Text input:"));

    let [state_1, state_2, state_3] = states;

    let text_field_1 = TextInput::new(state_1, "Row 1 text...", &strings[0], |content| {
        Message::TextChanged(content, 0)
    });
    let text_field_2 = TextInput::new(state_2, "Row 2 text...", &strings[1], |content| {
        Message::TextChanged(content, 1)
    });
    let text_field_3 = TextInput::new(state_3, "Row 3 text...", &strings[2], |content| {
        Message::TextChanged(content, 2)
    });

    content = content
        .push(text_field_1)
        .push(text_field_2)
        .push(text_field_3);

    content
}

fn add_slider_section<'a>(
    mut content: Column<'a, Message>,
    slider_states: &'a mut [RgbSlidersState; 3],
    color_values: &'a mut [RgbColor; 3],
) -> Column<'a, Message> {
    content = content.push(Text::new("Color selection:"));

    for (i, state) in slider_states.iter_mut().enumerate() {
        let slider_r = Slider::new(
            &mut state.r,
            COLOR_SLIDER_RANGE,
            color_values[i].r as i32,
            move |value| Message::ColorChanged(value, i, 0),
        );
        let slider_g = Slider::new(
            &mut state.g,
            COLOR_SLIDER_RANGE,
            color_values[i].g as i32,
            move |value| Message::ColorChanged(value, i, 1),
        );
        let slider_b = Slider::new(
            &mut state.b,
            COLOR_SLIDER_RANGE,
            color_values[i].b as i32,
            move |value| Message::ColorChanged(value, i, 2),
        );

        let color_preview = Canvas::new(Rect {
            x: -12.0,
            y: -12.0,
            color: color_values[i],
        })
        .width(Length::Units(25))
        .height(Length::Units(25));

        let slider_row = Row::new()
            .spacing(20)
            .push(Text::new("r"))
            .push(slider_r)
            .push(Text::new(color_values[i].r.to_string()))
            .push(Text::new("g"))
            .push(slider_g)
            .push(Text::new(color_values[i].g.to_string()))
            .push(Text::new("b"))
            .push(slider_b)
            .push(Text::new(color_values[i].b.to_string()))
            .push(color_preview);

        content = content
            .push(Text::new(String::from("Row ") + &i.to_string()))
            .push(slider_row);
    }

    content
}

fn add_anim_section<'a>(
    mut content: Column<'a, Message>,
    anim_states: &'a mut [iced::pick_list::State<Animation>; 3],
    anim_values: &'a mut [Option<Animation>; 3],
    speed_states: &'a mut [iced::text_input::State; 3],
    speed_values: &'a mut [String; 3],
    direction_states: &'a mut [iced::pick_list::State<Direction>; 3],
    direction_values: &'a mut [Option<Direction>; 3],
) -> Column<'a, Message> {
    content = content.push(Text::new("Animation selection:"));

    let [state_1, state_2, state_3] = anim_states;
    let [s_state_1, s_state_2, s_state_3] = speed_states;
    let [d_state_1, d_state_2, d_state_3] = direction_states;

    content = create_single_anim_section(
        content,
        0,
        state_1,
        anim_values[0],
        s_state_1,
        speed_values[0].clone(),
        d_state_1,
        direction_values[0],
    );
    content = create_single_anim_section(
        content,
        1,
        state_2,
        anim_values[1],
        s_state_2,
        speed_values[1].clone(),
        d_state_2,
        direction_values[1],
    );
    content = create_single_anim_section(
        content,
        2,
        state_3,
        anim_values[2],
        s_state_3,
        speed_values[2].clone(),
        d_state_3,
        direction_values[2],
    );

    content
}

fn create_single_anim_section<'a>(
    mut content: Column<'a, Message>,
    id: usize,
    anim_state: &'a mut iced::pick_list::State<Animation>,
    anim_value: Option<Animation>,
    speed_state: &'a mut iced::text_input::State,
    speed_value: String,
    direction_state: &'a mut iced::pick_list::State<Direction>,
    direction_value: Option<Direction>,
) -> Column<'a, Message> {
    let mut row = Row::new().spacing(20);

    let anim = PickList::new(
        anim_state,
        &Animation::ALL[..],
        anim_value,
        move |animation| Message::AnimChanged(animation, id),
    );

    row = row
        .push(Text::new(String::from("Row ") + &(id + 1).to_string()))
        .push(anim);

    if let Some(anim) = anim_value {
        match anim {
            Animation::None => {
                //do nothing
            }
            Animation::Blink => {
                let speed_text_field = TextInput::new(
                    speed_state,
                    "Animation speed (hz) (max 60)",
                    &speed_value,
                    move |content| Message::AnimSpeedChanged(content, id),
                );
                row = row.push(speed_text_field);
            }
            Animation::Slide => {
                let speed_text_field = TextInput::new(
                    speed_state,
                    "Animation speed (px/s) (max 60)",
                    &speed_value,
                    move |content| Message::AnimSpeedChanged(content, id),
                );
                row = row.push(speed_text_field);

                let direction_picker = PickList::new(
                    direction_state,
                    &Direction::ALL[..],
                    direction_value,
                    move |value| Message::AnimDirectionChanged(value, id),
                );

                row = row.push(direction_picker);
            }
        }
    }

    content = content.push(row);

    content
}

fn add_font_section<'a>(
    mut content: Column<'a, Message>,
    font_states: &'a mut [iced::pick_list::State<Font>; 3],
    font_values: &'a mut [Option<Font>; 3],
) -> Column<'a, Message> {
    content = content.push(Text::new("Font selection:"));

    for (i, (state, value)) in font_states.iter_mut().zip(font_values).enumerate() {
        let mut row = Row::new().spacing(20);

        let font_picker = PickList::new(state, &Font::ALL[..], *value, move |value| {
            Message::FontChanged(value, i)
        });

        row = row
            .push(Text::new(String::from("Row ") + &(i + 1).to_string()))
            .push(font_picker);

        content = content.push(row);
    }

    content
}
