use cursive::theme::{Color as CursiveColor, ColorStyle};
use cursive::vec::Vec2;
use cursive::Printer;

use rand::{Rand, Rng};

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

impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Color {
        use self::Color::*;
        lazy_static! {
            static ref COLORS: [Color; 6] =
                [Blue, Green, Orange, Pink, Red, White];
        }
        COLORS[rng.gen::<usize>() % 6]
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

impl Rand for Symbol {
    fn rand<R: Rng>(rng: &mut R) -> Symbol {
        use self::Symbol::*;
        lazy_static! {
            static ref SYMBOLS: [Symbol; 6] =
                [And, Carrot, Equals, Hash, Line, Star];
        }
        SYMBOLS[rng.gen::<usize>() % 6]
    }
}

pub struct Stone {
    pub color: Color,
    pub symbol: Symbol,
}

impl Stone {
    pub fn print(&self, pos: Vec2, printer: &Printer) {
        printer.with_color(self.color.to_color_style(), |p|
            p.print(pos, self.symbol.to_str())
        );
    }
}

impl Rand for Stone {
    fn rand<R: Rng>(rng: &mut R) -> Stone {
        Stone {
            color: Color::rand(rng),
            symbol: Symbol::rand(rng),
        }
    }
}

