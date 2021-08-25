use iced::{
    Application, Button, Clipboard, Column, Command, Container, Element, PickList, Row, Settings,
    Text, TextInput,
};
use serialport::SerialPort;

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
    
    AnimSelected(Animation),
    Ping,

    UpdateText1(String),
    UpdateText2(String),
    UpdateText3(String),
    SendText,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Animation{
	None,
	Slide,
	Blink
}

impl Animation {
    const ALL: [Animation; 3] = [
        Animation::None,
        Animation::Blink,
        Animation::Slide,
    ];
}

impl Default for Animation {
    fn default() -> Animation {
        Animation::None
    }
}

impl std::fmt::Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Animation::None => "None",
                Animation::Blink => "Blink",
                Animation::Slide => "Slide",
            }
        )
    }
}

struct App {
    state: AppState,
    connect_b: iced::button::State,
    refresh_b: iced::button::State,
    ping_b: iced::button::State,
    colors_b: iced::button::State,
    send_text_b: iced::button::State,
    
    port_list: Vec<String>,
    port_select_state: iced::pick_list::State<String>,
    selected_port: Option<String>,
    
    anim_select_state_1: iced::pick_list::State<Animation>,
    anim_select_state_2: iced::pick_list::State<Animation>,
    anim_select_state_3: iced::pick_list::State<Animation>,
    
    selected_anim_1: Option<Animation>,
    selected_anim_2: Option<Animation>,
    selected_anim_3: Option<Animation>,

    tistate_1: iced::text_input::State,
    tistate_2: iced::text_input::State,
    tistate_3: iced::text_input::State,
    text_row_1: String,
    text_row_2: String,
    text_row_3: String,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App {
                state: AppState::NotConnected,
                connect_b: iced::button::State::new(),
                refresh_b: iced::button::State::new(),
                ping_b: iced::button::State::new(),
                colors_b: iced::button::State::new(),
                send_text_b: iced::button::State::new(),
                
                port_list: vec![],
                port_select_state: iced::pick_list::State::default(),
                selected_port: None,
                
                anim_select_state_1: iced::pick_list::State::default(),
                anim_select_state_2: iced::pick_list::State::default(),
                anim_select_state_3: iced::pick_list::State::default(),
                selected_anim_1: Some(Animation::None),
                selected_anim_2: Some(Animation::None),
                selected_anim_3: Some(Animation::None),
                
                tistate_1: iced::text_input::State::new(),
                tistate_2: iced::text_input::State::new(),
                tistate_3: iced::text_input::State::new(),
                text_row_1: String::from(""),
                text_row_2: String::from(""),
                text_row_3: String::from(""),
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
                self.selected_port = Some(port);
            }
            Message::RefreshDevices => {
                enumerate_ports(&mut self.port_list);
            }
            Message::ConnectDevice => {
                if let Some(port) = self.selected_port.clone() {
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
            Message::UpdateText1(content) => self.text_row_1 = content.clone(),
            Message::UpdateText2(content) => self.text_row_2 = content.clone(),
            Message::UpdateText3(content) => self.text_row_3 = content.clone(),
            Message::SendText => self.send_text(),
            _ => {}
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let port_list = PickList::new(
            &mut self.port_select_state,
            self.port_list.clone(),
            self.selected_port.clone(),
            Message::PortSelected,
        );

        let refresh_button = Button::new(&mut self.refresh_b, Text::new("Refresh"))
            .on_press(Message::RefreshDevices);

        let mut connect_button = Button::new(&mut self.connect_b, Text::new("Connect"));
        let mut ping_button = Button::new(&mut self.ping_b, Text::new("Ping"));

        if let AppState::NotConnected = self.state {
            connect_button = connect_button.on_press(Message::ConnectDevice); 
        }else{
        	ping_button = ping_button.on_press(Message::Ping);
        }

        let controls = Row::new()
            .spacing(20)
            .push(port_list)
            .push(refresh_button)
            .push(connect_button)
            .push(ping_button);

        let mut content = Column::new().max_width(800).spacing(20).push(controls);

        if let AppState::DeviceConnected(_device) = &mut self.state {
            let text_field_1 = TextInput::new(
                &mut self.tistate_1,
                "Row 1 text...",
                &mut self.text_row_1,
                Message::UpdateText1,
            );
            let text_field_2 = TextInput::new(
                &mut self.tistate_2,
                "Row 2 text...",
                &mut self.text_row_2,
                Message::UpdateText2,
            );
            let text_field_3 = TextInput::new(
                &mut self.tistate_3,
                "Row 3 text...",
                &mut self.text_row_3,
                Message::UpdateText3,
            );
            
            let anim_1 = PickList::new(
            	&mut self.anim_select_state_1,
            	&Animation::ALL[..],
            	self.selected_anim_1,
            	Message::AnimSelected,
        	);
        	
        	let anim_2 = PickList::new(
            	&mut self.anim_select_state_2,
            	&Animation::ALL[..],
            	self.selected_anim_2,
            	Message::AnimSelected,
        	);
        	
        	let anim_3 = PickList::new(
            	&mut self.anim_select_state_3,
            	&Animation::ALL[..],
            	self.selected_anim_3,
            	Message::AnimSelected,
        	);

            let send_text_button = Button::new(&mut self.send_text_b, Text::new("Send text...")).on_press(Message::SendText);

			let text_1 = Row::new()
            .spacing(20).push(text_field_1).push(anim_1);
            
            let text_2 = Row::new()
            .spacing(20).push(text_field_2).push(anim_2);
            
            let text_3 = Row::new()
            .spacing(20).push(text_field_3).push(anim_3);

            content = content
                .push(text_1)
                .push(text_2)
                .push(text_3)
                .push(send_text_button);
        }
         

        Container::new(content).center_x().center_y().into()
    }
}

impl App {
    fn send_text(&mut self) {
        if self.text_row_1 != "" {
            //send via uart
            self.text_row_1 = String::from("");
        }
        if self.text_row_2 != "" {
            //send via uart
            self.text_row_2 = String::from("");
        }
        if self.text_row_3 != "" {

            //send via uart
            self.text_row_3 = String::from("");
        }
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

fn update_row_text(row: &mut String, content: String) {
    if content != "" {
        *row = content;
    }
}

