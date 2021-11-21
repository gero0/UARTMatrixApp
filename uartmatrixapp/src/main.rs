use iced::{
    Application, Button, Clipboard, Column, Command, Container, Element, PickList, Row, Settings,
    Text,
};
use image::imageops::FilterType;
use image::io::Reader;
use native_dialog::FileDialog;
use serialport::SerialPort;

use direct_mode_ui::{add_direct_mode_ui, DirectModeData};
use libuartmatrix::enums::DisplayMode;

use crate::{
    helper_structs::{Animation, Direction, Font},
    serial::*,
    text_mode_ui::{add_text_mode_ui, TextModeData},
    AppState::TextMode,
};

mod direct_mode_ui;
mod helper_structs;
mod rect;
mod serial;
mod text_mode_ui;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum AppState {
    NotConnected,
    TextMode,
    DirectMode,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeMode,
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
    //Direct mode
    DrawPixel,
    DrawLine,
    DrawRectangle,
    DrawTriangle,
    DrawCircle,
    ShapeColorChanged(i32, usize),
    ThicknessChanged(String),
    FilledChanged(bool),
    PixelCoordChanged(String, i32),
    LineCoordChanged(String, i32, i32),
    RectangleCoordChanged(String, i32, i32),
    TriangleCoordChanged(String, i32, i32),
    CircleCoordChanged(String, i32),
    ClearScreen,
}

struct App {
    state: AppState,

    connect_btn: iced::button::State,
    refresh_btn: iced::button::State,
    ping_btn: iced::button::State,
    change_mode_btn: iced::button::State,

    device: Option<Box<dyn SerialPort>>,
    port_list: Vec<String>,
    port_select_state: iced::pick_list::State<String>,
    port_select_value: Option<String>,

    text_mode_data: TextModeData,
    direct_mode_data: DirectModeData,
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
                change_mode_btn: iced::button::State::new(),

                device: None,

                port_list: vec![],
                port_select_state: iced::pick_list::State::default(),
                port_select_value: None,

                text_mode_data: TextModeData::new(),
                direct_mode_data: DirectModeData::new(),
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
                            self.state = AppState::TextMode;
                            self.device = Some(s_port);
                        }
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                }
            }

            Message::TextChanged(content, id) => {
                let text_row = &mut self.text_mode_data.text_rows_values[id];
                *text_row = content;
            }

            Message::AnimChanged(animation, id) => {
                let anim = &mut self.text_mode_data.anim_select_values[id];
                *anim = Some(animation);
            }

            Message::ColorChanged(value, row, color) => {
                let mut row = &mut self.text_mode_data.color_slider_values[row];
                match color {
                    0 => row.r = value as u8,
                    1 => row.g = value as u8,
                    2 => row.b = value as u8,
                    _ => panic!("Invalid color index!"),
                }
            }

            Message::AnimSpeedChanged(value, row) => {
                let row = &mut self.text_mode_data.anim_speed_values[row];
                *row = value;
            }

            Message::AnimDirectionChanged(direction, row) => {
                let row = &mut self.text_mode_data.anim_direction_values[row];
                *row = Some(direction);
            }

            Message::FontChanged(font, row) => {
                let row = &mut self.text_mode_data.font_values[row];
                *row = Some(font);
            }

            Message::SendText => {
                if let TextMode = &mut self.state {
                    send_text(
                        self.device.as_mut().unwrap().as_mut(),
                        &self.text_mode_data.text_rows_values,
                    );
                }
            }

            Message::SendColors => {
                if let TextMode = &mut self.state {
                    send_colors(
                        self.device.as_mut().unwrap().as_mut(),
                        &self.text_mode_data.color_slider_values,
                    );
                }
            }

            Message::SendAnims => {
                if let TextMode = &mut self.state {
                    send_animations(
                        self.device.as_mut().unwrap().as_mut(),
                        &self.text_mode_data.anim_select_values,
                        &self.text_mode_data.anim_speed_values,
                        &self.text_mode_data.anim_direction_values,
                    );
                }
            }

            Message::SendFonts => {
                if let TextMode = &mut self.state {
                    send_fonts(
                        self.device.as_mut().unwrap().as_mut(),
                        &self.text_mode_data.font_values,
                    );
                }
            }

            Message::ChangeMode => {
                match &mut self.state {
                    AppState::TextMode => {
                        //send command to switch mode
                        self.state = AppState::DirectMode;
                        send_change_mode(
                            self.device.as_mut().unwrap().as_mut(),
                            DisplayMode::Direct,
                        );
                    }
                    AppState::DirectMode => {
                        //send command to switch mode
                        self.state = AppState::TextMode;
                        send_change_mode(self.device.as_mut().unwrap().as_mut(), DisplayMode::Text);
                    }
                    _ => {}
                }
            }

            Message::LoadImage => {
                let path = FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .show_open_single_file()
                    .unwrap();

                if let Some(path) = path {
                    let result = Reader::open(path);
                    match result {
                        Ok(img) => {
                            let img = img.decode();
                            if let Ok(img) = img {
                                let img = img.resize(64, 32, FilterType::Lanczos3);
                                let img = img.into_rgb8();
                                send_image(self.device.as_mut().unwrap().as_mut(), img);
                            }
                        }
                        Err(_e) => println!("Error opening file"),
                    }
                };
            }

            Message::FilledChanged(state) => {
                self.direct_mode_data.filled_value = state;
            }

            Message::ThicknessChanged(value) => {
                self.direct_mode_data.shape_thickness_value = value;
            }

            Message::ShapeColorChanged(value, color) => {
                let mut values = &mut self.direct_mode_data.color_slider_values;
                match color {
                    0 => values.r = value as u8,
                    1 => values.g = value as u8,
                    2 => values.b = value as u8,
                    _ => panic!("Invalid color index!"),
                }
            }

            Message::PixelCoordChanged(value, coord) => {
                if coord == 0 {
                    self.direct_mode_data.pixel_x_text_input = value
                } else {
                    self.direct_mode_data.pixel_y_text_input = value
                }
            }

            Message::RectangleCoordChanged(value, point, coord) => match point {
                0 => match coord {
                    0 => self.direct_mode_data.rectangle_x_1_text_input = value,
                    _ => self.direct_mode_data.rectangle_y_1_text_input = value,
                },
                _ => match coord {
                    0 => self.direct_mode_data.rectangle_x_2_text_input = value,
                    _ => self.direct_mode_data.rectangle_y_2_text_input = value,
                },
            },

            Message::LineCoordChanged(value, point, coord) => match point {
                0 => match coord {
                    0 => self.direct_mode_data.line_x_1_text_input = value,
                    _ => self.direct_mode_data.line_y_1_text_input = value,
                },
                _ => match coord {
                    0 => self.direct_mode_data.line_x_2_text_input = value,
                    _ => self.direct_mode_data.line_y_2_text_input = value,
                },
            },

            Message::TriangleCoordChanged(value, point, coord) => match point {
                0 => match coord {
                    0 => self.direct_mode_data.triangle_x_1_text_input = value,
                    _ => self.direct_mode_data.triangle_y_1_text_input = value,
                },
                1 => match coord {
                    0 => self.direct_mode_data.triangle_x_2_text_input = value,
                    _ => self.direct_mode_data.triangle_y_2_text_input = value,
                },
                _ => match coord {
                    0 => self.direct_mode_data.triangle_x_3_text_input = value,
                    _ => self.direct_mode_data.triangle_y_3_text_input = value,
                },
            },

            Message::CircleCoordChanged(value, point) => match point {
                0 => self.direct_mode_data.circle_x_text_input = value,
                1 => self.direct_mode_data.circle_y_text_input = value,
                _ => self.direct_mode_data.circle_radius_text_input = value,
            },

            Message::DrawPixel => {
                send_draw_pixel(
                    self.device.as_mut().unwrap().as_mut(),
                    &self.direct_mode_data.pixel_x_text_input,
                    &self.direct_mode_data.pixel_y_text_input,
                    &self.direct_mode_data.color_slider_values,
                );
            }

            Message::DrawLine => {
                send_draw_line(
                    self.device.as_mut().unwrap().as_mut(),
                    &self.direct_mode_data.line_x_1_text_input,
                    &self.direct_mode_data.line_y_1_text_input,
                    &self.direct_mode_data.line_x_2_text_input,
                    &self.direct_mode_data.line_y_2_text_input,
                    &self.direct_mode_data.color_slider_values,
                    &self.direct_mode_data.shape_thickness_value,
                );
            }

            Message::DrawRectangle => {
                send_draw_rectangle(
                    self.device.as_mut().unwrap().as_mut(),
                    &self.direct_mode_data.rectangle_x_1_text_input,
                    &self.direct_mode_data.rectangle_y_1_text_input,
                    &self.direct_mode_data.rectangle_x_2_text_input,
                    &self.direct_mode_data.rectangle_y_2_text_input,
                    &self.direct_mode_data.color_slider_values,
                    &self.direct_mode_data.shape_thickness_value,
                    self.direct_mode_data.filled_value,
                );
            }

            Message::DrawTriangle => {
                send_draw_triangle(
                    self.device.as_mut().unwrap().as_mut(),
                    &self.direct_mode_data.triangle_x_1_text_input,
                    &self.direct_mode_data.triangle_y_1_text_input,
                    &self.direct_mode_data.triangle_x_2_text_input,
                    &self.direct_mode_data.triangle_y_2_text_input,
                    &self.direct_mode_data.triangle_x_3_text_input,
                    &self.direct_mode_data.triangle_y_3_text_input,
                    &self.direct_mode_data.color_slider_values,
                    &self.direct_mode_data.shape_thickness_value,
                    self.direct_mode_data.filled_value,
                );
            }

            Message::DrawCircle => {
                send_draw_circle(
                    self.device.as_mut().unwrap().as_mut(),
                    &self.direct_mode_data.circle_x_text_input,
                    &self.direct_mode_data.circle_y_text_input,
                    &self.direct_mode_data.circle_radius_text_input,
                    &self.direct_mode_data.color_slider_values,
                    &self.direct_mode_data.shape_thickness_value,
                    self.direct_mode_data.filled_value,
                );
            }

            Message::ClearScreen => {
                send_clear_screen(self.device.as_mut().unwrap().as_mut());
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
        // let mut ping_button = Button::new(&mut self.ping_btn, Text::new("Ping"));
        let mut change_mode_button =
            Button::new(&mut self.change_mode_btn, Text::new("Change display mode"));
        if let AppState::NotConnected = self.state {
            connect_button = connect_button.on_press(Message::ConnectDevice);
        } else {
            // ping_button = ping_button.on_press(Message::Ping);
            change_mode_button = change_mode_button.on_press(Message::ChangeMode);
        }

        let controls = Row::new()
            .spacing(20)
            .push(port_list)
            .push(refresh_button)
            .push(connect_button)
            // .push(ping_button)
            .push(change_mode_button);

        let mut content = Column::new().push(controls).spacing(20);

        match &mut self.state {
            AppState::TextMode => {
                let (left_column, right_column) = add_text_mode_ui(&mut self.text_mode_data);
                let text_ui = Row::new().push(left_column).push(right_column).spacing(20);
                content = content.push(text_ui);
            }
            AppState::DirectMode => {
                let (left_column, right_column) = add_direct_mode_ui(&mut self.direct_mode_data);
                let ui = Row::new().push(left_column).push(right_column).spacing(20);
                content = content.push(ui);
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
