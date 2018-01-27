use cursive::theme::{Color as CursiveColor, ColorStyle};
use cursive::vec::Vec2;
use cursive::Printer;

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Orange,
    Yellow,
    White,
}

impl Color {
    fn to_color_style(self) -> ColorStyle {
        use self::Color::*;
        use self::CursiveColor::*;
        let fg_color = match self {
            Red => RgbLowRes(5, 0, 0),
            Green => RgbLowRes(0, 5, 0),
            Blue => RgbLowRes(0, 0, 5),
            Orange => RgbLowRes(5, 3, 0),
            Yellow => RgbLowRes(4, 4, 0),
            White => RgbLowRes(5, 5, 5),
        };
        ColorStyle::Custom {
            front: CursiveColor::TerminalDefault,
            back: fg_color,
        }
    }
}

// TODO: use nifty unicode chars
#[derive(Copy, Clone)]
pub enum Symbol {
    Star,
    Carrot,
    // technically an octothorpe...
    Hash, 
    Line,
    Equals,
    And,
}

impl Symbol {
    fn to_str(self) -> &'static str {
        use self::Symbol::*;
        match self {
            Star => "*",
            Carrot => "^",
            Hash => "#",
            Line => "|",
            Equals => "=",
            And => "&",
        }
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

