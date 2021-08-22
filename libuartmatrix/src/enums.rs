pub enum DisplayMode {
    Text,
    Direct,
}

impl Into<u8> for DisplayMode {
    fn into(self) -> u8 {
        match self {
            DisplayMode::Text => 0,
            DisplayMode::Direct => 1,
        }
    }
}

pub enum FontType {
    Default,
    Pro,
    Ibm,
}

impl Into<u8> for FontType {
    fn into(self) -> u8 {
        match self {
            FontType::Default => 0,
            FontType::Pro => 1,
            FontType::Ibm => 2,
        }
    }
}

pub enum Direction {
    Left,
    Right,
}

pub enum Animation {
    NoAnimation,
    BlinkAnimation(u8),
    SlideAnimation(u8, Direction),
}

impl Into<Vec<u8>> for Animation {
    fn into(self) -> Vec<u8> {
        match self {
            Animation::NoAnimation => vec![0_u8],
            Animation::BlinkAnimation(speed) => vec![1_u8, speed],
            Animation::SlideAnimation(speed, direction) => {
                let dir = match direction {
                    Direction::Left => 0,
                    Direction::Right => 1,
                };

                vec![2_u8, speed, dir]
            }
        }
    }
}