use std::marker::PhantomData;
use std::ops::RangeInclusive;

use iced::{
    canvas::{self, Cursor, Frame, Geometry, Path, Program, Stroke},
    slider, Align, Application, Button, Canvas, Clipboard, Color, Column, Command, Container,
    Element, HorizontalAlignment, Length, PickList, Point, Rectangle, Row, Sandbox, Settings, Size,
    Slider, Text, TextInput, Vector, VerticalAlignment,
};
use serialport::SerialPort;

use crate::helper_structs::{Direction, Font};
use crate::AppState::DeviceConnected;
use crate::{
    helper_structs::{Animation, RgbColor, RgbSlidersState},
    rect::Rect,
    serial::*,
};

mod helper_structs;
mod rect;
mod serial;

const COLOR_SLIDER_RANGE: RangeInclusive<i32> = (0..=255);

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum AppState {
    NotConnected,
    DeviceConnected(Box<dyn SerialPort>),
}

#[derive(Debug, Clone)]
enum Message {
    PortSelected(String),
    RefreshDevices,
    ConnectDevice,
    LoadImage,
    Ping,
    TextChanged(String, usize),
    AnimChanged(Animation, usize),
    AnimSpeedChanged(String, usize),
    AnimDirectionChanged(Direction, usize),
    ColorChanged(i32, usize, usize),
    FontChanged(Font, usize),
    SendText,
    SendColors,
    SendAnims,
    SendFonts,
}

struct App {
    state: AppState,

    connect_btn: iced::button::State,
    refresh_btn: iced::button::State,
    ping_btn: iced::button::State,
    send_colors_btn: iced::button::State,
    send_anims_btn: iced::button::State,
    load_btn: iced::button::State,
    send_text_btn: iced::button::State,
    send_fonts_btn: iced::button::State,

    port_list: Vec<String>,
    port_select_state: iced::pick_list::State<String>,
    port_select_value: Option<String>,

    anim_select_states: [iced::pick_list::State<Animation>; 3],
    anim_select_values: [Option<Animation>; 3],

    anim_speed_states: [iced::text_input::State; 3],
    anim_speed_values: [String; 3],

    anim_direction_states: [iced::pick_list::State<Direction>; 3],
    anim_direction_values: [Option<Direction>; 3],

    font_states: [iced::pick_list::State<Font>; 3],
    font_values: [Option<Font>; 3],

    text_rows_states: [iced::text_input::State; 3],
    text_rows_values: [String; 3],

    color_slider_states: [RgbSlidersState; 3],
    color_slider_values: [RgbColor; 3],
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App {
                state: AppState::NotConnected,
                connect_btn: iced::button::State::new(),
                refresh_btn: iced::button::State::new(),
                ping_btn: iced::button::State::new(),
                send_colors_btn: iced::button::State::new(),
                load_btn: iced::button::State::new(),
                send_text_btn: iced::button::State::new(),
                send_anims_btn: iced::button::State::new(),
                send_fonts_btn: iced::button::State::new(),

                port_list: vec![],
                port_select_state: iced::pick_list::State::default(),
                port_select_value: None,

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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("UART Matrix test app")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::PortSelected(port) => {
                self.port_select_value = Some(port);
            }
            Message::RefreshDevices => {
                enumerate_ports(&mut self.port_list);
            }
            Message::ConnectDevice => {
                if let Some(port) = self.port_select_value.clone() {
                    let sp = serialport::new(port, 115200).open();
                    match sp {
                        Ok(s_port) => {
                            self.state = AppState::DeviceConnected(s_port);
                        }
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                }
            }

            Message::TextChanged(content, id) => {
                let mut text_row = &mut self.text_rows_values[id];
                *text_row = content;
            }

            Message::AnimChanged(animation, id) => {
                let mut anim = &mut self.anim_select_values[id];
                *anim = Some(animation);
            }

            Message::ColorChanged(value, row, color) => {
                let mut row = &mut self.color_slider_values[row];
                match color {
                    0 => row.r = value,
                    1 => row.g = value,
                    2 => row.b = value,
                    _ => panic!("Invalid color index!"),
                }
            }

            Message::AnimSpeedChanged(value, row) => {
                let mut row = &mut self.anim_speed_values[row];
                *row = value;
            }

            Message::AnimDirectionChanged(direction, row) => {
                let mut row = &mut self.anim_direction_values[row];
                *row = Some(direction);
            }

            Message::FontChanged(font, row) => {
                let mut row = &mut self.font_values[row];
                *row = Some(font);
            }

            Message::SendText => {
                if let DeviceConnected(device) = &mut self.state {
                    send_text(device.as_mut(), &self.text_rows_values);
                }
            }

            Message::SendColors => {
                if let DeviceConnected(device) = &mut self.state {
                    send_colors(device.as_mut(), &self.color_slider_values);
                }
            }

            Message::SendAnims => {
                if let DeviceConnected(device) = &mut self.state {
                    send_animations(
                        device.as_mut(),
                        &self.anim_select_values,
                        &self.anim_speed_values,
                        &self.anim_direction_values,
                    );
                }
            }

            Message::SendFonts => {
                if let DeviceConnected(device) = &mut self.state {
                    send_fonts(device.as_mut(), &self.font_values);
                }
            }

            _ => {}
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let port_list = PickList::new(
            &mut self.port_select_state,
            self.port_list.clone(),
            self.port_select_value.clone(),
            Message::PortSelected,
        );

        let refresh_button = Button::new(&mut self.refresh_btn, Text::new("Refresh"))
            .on_press(Message::RefreshDevices);

        let mut connect_button = Button::new(&mut self.connect_btn, Text::new("Connect"));
        let mut ping_button = Button::new(&mut self.ping_btn, Text::new("Ping"));
        let mut load_image_button = Button::new(&mut self.load_btn, Text::new("Load image..."));

        if let AppState::NotConnected = self.state {
            connect_button = connect_button.on_press(Message::ConnectDevice);
        } else {
            ping_button = ping_button.on_press(Message::Ping);
            load_image_button = load_image_button.on_press(Message::LoadImage);
        }

        let controls = Row::new()
            .spacing(20)
            .push(port_list)
            .push(refresh_button)
            .push(connect_button)
            .push(ping_button)
            .push(load_image_button);

        let mut content = Column::new().max_width(800).spacing(20).push(controls);

        if let AppState::DeviceConnected(_device) = &mut self.state {
            content = Self::add_text_section(
                content,
                &mut self.text_rows_states,
                &mut self.text_rows_values,
            );

            let send_text_button = Button::new(&mut self.send_text_btn, Text::new("Send text"))
                .on_press(Message::SendText);

            content = content.push(send_text_button);

            content = Self::add_slider_section(
                content,
                &mut self.color_slider_states,
                &mut self.color_slider_values,
            );

            let mut send_colors_button =
                Button::new(&mut self.send_colors_btn, Text::new("Set colors"))
                    .on_press(Message::SendColors);

            content = content.push(send_colors_button);

            content = Self::add_anim_section(
                content,
                &mut self.anim_select_states,
                &mut self.anim_select_values,
                &mut self.anim_speed_states,
                &mut self.anim_speed_values,
                &mut self.anim_direction_states,
                &mut self.anim_direction_values,
            );

            let mut send_anims_button =
                Button::new(&mut self.send_anims_btn, Text::new("Set animations"))
                    .on_press(Message::SendAnims);

            content = content.push(send_anims_button);

            content = Self::add_font_section(content, &mut self.font_states, &mut self.font_values);

            let mut send_fonts_button =
                Button::new(&mut self.send_fonts_btn, Text::new("Set fonts"))
                    .on_press(Message::SendFonts);

            content = content.push(send_fonts_button);
        }

        Container::new(content)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

impl App {
    fn add_text_section<'a>(
        mut content: Column<'a, Message>,
        states: &'a mut [iced::text_input::State; 3],
        strings: &'a mut [String; 3],
    ) -> Column<'a, Message> {
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

        return content;
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
                color_values[i].r,
                move |value| Message::ColorChanged(value, i, 0),
            );
            let slider_g = Slider::new(
                &mut state.g,
                COLOR_SLIDER_RANGE,
                color_values[i].g,
                move |value| Message::ColorChanged(value, i, 1),
            );
            let slider_b = Slider::new(
                &mut state.b,
                COLOR_SLIDER_RANGE,
                color_values[i].b,
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

        return content;
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

        content = Self::create_single_anim_section(
            content,
            0,
            state_1,
            anim_values[0],
            s_state_1,
            speed_values[0].clone(),
            d_state_1,
            direction_values[0],
        );
        content = Self::create_single_anim_section(
            content,
            1,
            state_2,
            anim_values[1],
            s_state_2,
            speed_values[1].clone(),
            d_state_2,
            direction_values[1],
        );
        content = Self::create_single_anim_section(
            content,
            2,
            state_3,
            anim_values[2],
            s_state_3,
            speed_values[2].clone(),
            d_state_3,
            direction_values[2],
        );

        return content;
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
            if let Animation::None = anim {
                //Do nothing
            } else {
                let speed_text_field = TextInput::new(
                    speed_state,
                    "Animation speed",
                    &speed_value,
                    move |content| Message::AnimSpeedChanged(content, id),
                );
                row = row.push(speed_text_field);

                if let Animation::Slide = anim {
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

        return content;
    }

    fn add_font_section<'a>(
        mut content: Column<'a, Message>,
        font_states: &'a mut [iced::pick_list::State<Font>; 3],
        font_values: &'a mut [Option<Font>; 3],
    ) -> Column<'a, Message> {
        content = content.push(Text::new("Font selection:"));

        for (i, (state, value)) in font_states.iter_mut().zip(font_values).enumerate() {
            let mut row = Row::new().spacing(20);

            let font_picker = PickList::new(state, &Font::ALL[..], value.clone(), move |value| {
                Message::FontChanged(value, i)
            });

            row = row
                .push(Text::new(String::from("Row ") + &(i + 1).to_string()))
                .push(font_picker);

            content = content.push(row);
        }

        return content;
    }
}

fn enumerate_ports(port_list: &mut Vec<String>) {
    let ports = serialport::available_ports();
    if let Ok(ports) = ports {
        *port_list = ports
            .iter()
            .map(|port_info| port_info.port_name.clone())
            .collect();
    }
}
