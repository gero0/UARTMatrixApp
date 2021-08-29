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
pub enum Direction {
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Left
    }
}

impl Direction {
    pub const ALL: [Direction; 2] = [Direction::Left, Direction::Right];
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => "Left",
                Direction::Right => "Right",
            }
        )
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Font {
    Default,
    Pro,
    Ibm,
}

impl Font {
    pub const ALL: [Font; 3] = [Font::Default, Font::Pro, Font::Ibm];
}

impl Default for Font {
    fn default() -> Font {
        Font::Default
    }
}

impl std::fmt::Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Font::Default => "Default",
                Font::Pro => "ProFont",
                Font::Ibm => "IBM",
            }
        )
    }
}
