use cursive::view::View;
use cursive::Printer;
use cursive::vec::Vec2;
use cursive::event::{Event, EventResult};
use cursive::theme::ColorStyle;
use cursive::theme::Color::RgbLowRes;

use rand::{self, Rng};

use stone::{Stone, Color, Symbol};

const INSET: (usize, usize) = (1, 1);
const BOARD_SIZE: (usize, usize) = (12, 8);
const VIEW_SIZE: (usize, usize) = (BOARD_SIZE.0 + INSET.0 * 2, BOARD_SIZE.1 + INSET.1 * 2);

pub struct Board {
    cells: [[Option<Stone>; 12]; 8],
    cells_inset: Vec2,
    cursor_pos: Vec2,
}

impl Board {
    fn draw_cell(&self, pos: Vec2, printer: &Printer) {
        let cell = self.cells[pos.y][pos.x].as_ref();

        let c = if pos == self.cursor_pos {
            ColorStyle::Custom {
                front: RgbLowRes(5, 5, 5),
                back: RgbLowRes(2, 2, 2),
            }
        } else {
            cell.map_or(ColorStyle::Primary, |c| c.color.to_color_style())
        };

        let s = if pos == self.cursor_pos {
            "?"
        } else {
            cell.map_or(".", |c| c.symbol.to_str())
        };

        printer.with_color(c, |p| p.print(pos + INSET, s));
    }

    pub fn new() -> Board {
        let mut b = Board {
            cells: Default::default(),
            cells_inset: Vec2::new(INSET.0, INSET.1),
            cursor_pos: Vec2::new(0, 0),
        };
        b.init();
        b
    }

    fn init(&mut self) {
        use stone::Symbol::*;
        use stone::Color::*;
        let mut colors = [Blue, Green, Orange, Pink, Red, White];
        let mut symbols = [And, Carrot, Equals, Hash, Line, Star];
        let coords = [(0, 0), (11, 0), (0, 7), (11, 7), (5, 3), (6, 4)];
        rand::thread_rng().shuffle(&mut colors);
        rand::thread_rng().shuffle(&mut symbols);
        let iter = colors.iter().zip(symbols.iter()).zip(coords.iter());
        for ((&color, &symbol), &(x, y)) in iter {
            self.cells[y][x] = Some(Stone { color, symbol });
        }
    }
}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        for y in 0..BOARD_SIZE.1 {
            for x in 0..BOARD_SIZE.0 {
                self.draw_cell(Vec2::new(x, y), printer);
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(VIEW_SIZE.0, VIEW_SIZE.1)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        use cursive::event::Key::*;
        if let Event::Key(k) = event {
            match k {
                Left => self.cursor_pos.x -= 1,
                Right => self.cursor_pos.x += 1,
                Up => self.cursor_pos.y -= 1,
                Down => self.cursor_pos.y += 1,
                _ => (),
            }
        }
        EventResult::Ignored
    }
}

