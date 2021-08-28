#[derive(Debug, Copy, Clone)]
pub struct RgbColor {
    pub r: i32,
    pub g: i32,
    pub b: i32,
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

#[derive(Debug, Copy, Clone)]
pub struct RgbSlidersState {
    pub r: iced::slider::State,
    pub g: iced::slider::State,
    pub b: iced::slider::State,
}

impl RgbSlidersState {
    pub fn new() -> Self {
        RgbSlidersState {
            r: iced::slider::State::new(),
            g: iced::slider::State::new(),
            b: iced::slider::State::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Animation {
    None,
    Slide,
    Blink,
}

impl Animation {
    pub const ALL: [Animation; 3] = [Animation::None, Animation::Blink, Animation::Slide];
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
