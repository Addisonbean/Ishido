use cursive::theme::{Color as CursiveColor, ColorStyle};
use cursive::vec::Vec2;
use cursive::Printer;

#[derive(Copy, Clone)]
pub enum Color {
    Blue,
    Green,
    Orange,
    Pink,
    Red,
    White,
}

impl Color {
    pub fn to_color_style(self) -> ColorStyle {
        use self::Color::*;
        use self::CursiveColor::*;
        let (fg_color, black_text) = match self {
            Blue => (RgbLowRes(0, 1, 4), false),
            Green => (RgbLowRes(0, 5, 1), false),
            Orange => (RgbLowRes(5, 4, 0), true),
            Pink => (RgbLowRes(5, 1, 3), true),
            Red => (RgbLowRes(5, 0, 0), false),
            White => (RgbLowRes(5, 5, 5), true),
        };
        ColorStyle::Custom {
            front: if black_text {
                RgbLowRes(0, 0, 0)
            } else {
                RgbLowRes(5, 5, 5)
            },
            back: fg_color,
        }
    }
}

// TODO: use nifty unicode chars
#[derive(Copy, Clone)]
pub enum Symbol {
    And,
    Carrot,
    Equals,
    // technically it's an octothorpe...
    Hash, 
    Line,
    Star,
}

impl Symbol {
    pub fn to_str(self) -> &'static str {
        use self::Symbol::*;
        match self {
            And => "&",
            Carrot => "^",
            Equals => "=",
            Hash => "#",
            Line => "|",
            Star => "*",
        }
    }
}

pub struct Stone {
    pub color: Color,
    pub symbol: Symbol,
}

