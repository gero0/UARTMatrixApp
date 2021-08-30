use iced::{
    Application, Button, Clipboard, Column, Command, Container, Element, PickList, Row, Settings,
    Text,
};
use serialport::SerialPort;

use crate::{
    helper_structs::{Animation, Direction, Font},
    serial::*,
    text_mode_ui::{add_text_mode_ui, TextModeData},
    AppState::TextMode,
};

mod helper_structs;
mod rect;
mod serial;
mod text_mode_ui;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum AppState {
    NotConnected,
    TextMode(Box<dyn SerialPort>),
    DirectMode(Box<dyn SerialPort>),
}

#[derive(Debug, Clone)]
pub enum Message {
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
    load_btn: iced::button::State,

    port_list: Vec<String>,
    port_select_state: iced::pick_list::State<String>,
    port_select_value: Option<String>,

    text_mode_data: TextModeData,
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

                load_btn: iced::button::State::new(),

                port_list: vec![],
                port_select_state: iced::pick_list::State::default(),
                port_select_value: None,

                text_mode_data: TextModeData::new(),
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
                            self.state = AppState::TextMode(s_port);
                        }
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                }
            }

            Message::TextChanged(content, id) => {
                let mut text_row = &mut self.text_mode_data.text_rows_values[id];
                *text_row = content;
            }

            Message::AnimChanged(animation, id) => {
                let mut anim = &mut self.text_mode_data.anim_select_values[id];
                *anim = Some(animation);
            }

            Message::ColorChanged(value, row, color) => {
                let mut row = &mut self.text_mode_data.color_slider_values[row];
                match color {
                    0 => row.r = value,
                    1 => row.g = value,
                    2 => row.b = value,
                    _ => panic!("Invalid color index!"),
                }
            }

            Message::AnimSpeedChanged(value, row) => {
                let mut row = &mut self.text_mode_data.anim_speed_values[row];
                *row = value;
            }

            Message::AnimDirectionChanged(direction, row) => {
                let mut row = &mut self.text_mode_data.anim_direction_values[row];
                *row = Some(direction);
            }

            Message::FontChanged(font, row) => {
                let mut row = &mut self.text_mode_data.font_values[row];
                *row = Some(font);
            }

            Message::SendText => {
                if let TextMode(device) = &mut self.state {
                    send_text(device.as_mut(), &self.text_mode_data.text_rows_values);
                }
            }

            Message::SendColors => {
                if let TextMode(device) = &mut self.state {
                    send_colors(device.as_mut(), &self.text_mode_data.color_slider_values);
                }
            }

            Message::SendAnims => {
                if let TextMode(device) = &mut self.state {
                    send_animations(
                        device.as_mut(),
                        &self.text_mode_data.anim_select_values,
                        &self.text_mode_data.anim_speed_values,
                        &self.text_mode_data.anim_direction_values,
                    );
                }
            }

            Message::SendFonts => {
                if let TextMode(device) = &mut self.state {
                    send_fonts(device.as_mut(), &self.text_mode_data.font_values);
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

        if let AppState::NotConnected = self.state {
            connect_button = connect_button.on_press(Message::ConnectDevice);
        } else {
            ping_button = ping_button.on_press(Message::Ping);
        }

        let controls = Row::new()
            .spacing(20)
            .push(port_list)
            .push(refresh_button)
            .push(connect_button)
            .push(ping_button);

        let mut content = Column::new().push(controls).spacing(20);

        match &mut self.state {
            AppState::TextMode(_device) => {
                let (left_column, right_column) = add_text_mode_ui(&mut self.text_mode_data);
                let text_ui = Row::new().push(left_column).push(right_column).spacing(20);
                content = content.push(text_ui);
            }
            _ => {}
        }

        Container::new(content)
            .center_x()
            .center_y()
            .padding(10)
            .into()
    }
}

impl App {}

fn enumerate_ports(port_list: &mut Vec<String>) {
    let ports = serialport::available_ports();
    if let Ok(ports) = ports {
        *port_list = ports
            .iter()
            .map(|port_info| port_info.port_name.clone())
            .collect();
    }
}
