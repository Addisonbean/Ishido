use std::cmp;

use cursive::view::View;
use cursive::Printer;
use cursive::vec::Vec2;
use cursive::event::{Event, EventResult};
use cursive::theme::ColorStyle;
use cursive::theme::Color::RgbLowRes;

use rand::{self, Rng};

use permutohedron::Heap;

use stone::Stone;

const INSET: (usize, usize) = (1, 1);
const BOARD_SIZE: (usize, usize) = (12, 8);
const VIEW_SIZE: (usize, usize) = (BOARD_SIZE.0 + INSET.0 * 2 + 20, BOARD_SIZE.1 + INSET.1 * 2);

const MATCH_BONUSES: [u64; 12] = [25, 50, 100, 200, 400, 600, 800, 1000, 5000,
    10000, 25000, 50000];

fn clamp<T: Ord>(n: T, min: T, max: T) -> T {
    cmp::min(cmp::max(min, n), max)
}

pub struct Board {
    cells: [[Option<Stone>; 12]; 8],
    cursor_pos: Vec2,
    stones: Vec<Stone>,
    score: u64,
    multiplier_exp: u64,
    game_over: bool,
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
            cursor_pos: Vec2::new(0, 0),
            stones: Vec::with_capacity(72),
            score: 0,
            multiplier_exp: 0,
            game_over: false,
        };
        b.init();
        b
    }

    fn init(&mut self) {
        use stone::Symbol::*;
        use stone::Color::*;
        let symbols = &[And, Carrot, Equals, Hash, Line, Star];
        let colors = &[Blue, Green, Orange, Pink, Red, White];
        for &symbol in symbols {
            for &color in colors {
                self.stones.push(Stone { symbol, color });
                self.stones.push(Stone { symbol, color });
            }
        }

        let mut colors_found = Vec::with_capacity(6);
        let mut symbols_found = Vec::with_capacity(6);
        let coords = [(0, 0), (11, 0), (0, 7), (11, 7), (5, 3), (6, 4)].iter();
        rand::thread_rng().shuffle(&mut self.stones);
        for &(x, y) in coords {
            let i = self.stones.iter().position(|s| !symbols_found.contains(&s.symbol) && !colors_found.contains(&s.color)).unwrap();
            let s = self.stones.swap_remove(i);
            symbols_found.push(s.symbol);
            colors_found.push(s.color);
            self.cells[y][x] = Some(s);
        }
    }

    fn place_stone(&mut self) -> Result<bool, MoveError> {
        let Vec2 { x, y } = self.cursor_pos;
        if self.cells[y][x].is_some() {
            Err(MoveError::DoublePlace)
        } else if let Some(s) = self.stones.pop() {
            let (valid, neighbors) = self.is_valid_move(self.cursor_pos, &s);
            if valid {
                self.cells[y][x] = Some(s);
                self.score += 2u64.pow(neighbors as u32 + self.multiplier_exp as u32 - 1);
                if neighbors == 4 {
                    self.score += MATCH_BONUSES[self.multiplier_exp as usize];
                    self.multiplier_exp += 1;
                }
                Ok(self.is_game_over())
            } else {
                self.stones.push(s);
                Err(MoveError::InvalidMove)
            }
        } else {
            Err(MoveError::StackEmpty)
        }
    }

    fn is_valid_move(&self, pos: Vec2, stone: &Stone) -> (bool, usize) {
        let mut neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().filter_map(|&(x, y)|
            match (pos.x as isize + x, pos.y as isize + y) {
                (x, y) if x >= 0 && x < BOARD_SIZE.0 as isize && y >= 0 && y < BOARD_SIZE.1 as isize =>
                    self.cells[y as usize][x as usize].as_ref(),
                _ => None,
            }
        ).collect::<Vec<_>>();
        let cond = match neighbors.len() {
            0 => false,
            1 => neighbors[0].color == stone.color ||
                 neighbors[0].symbol == stone.symbol,
            2 => Heap::new(&mut neighbors).any(|n|
                n[0].color == stone.color && n[1].symbol == stone.symbol
            ),
            3 => Heap::new(&mut neighbors).any(|n|
                (n[0].color == stone.color && n[1].color == stone.color &&
                 n[2].symbol == stone.symbol) ||
                (n[0].symbol == stone.symbol && n[1].symbol == stone.symbol &&
                 n[2].color == stone.color)
            ),
            4 => Heap::new(&mut neighbors).any(|n|
                n[0].color == stone.color && n[1].color == stone.color &&
                n[2].symbol == stone.symbol && n[3].symbol == stone.symbol
            ),
            _ => unreachable!(),
        };
        (cond, neighbors.len())
    }

    fn is_game_over(&self) -> bool {
        if let Some(next_stone) = self.peek_next_stone() {
            for (y, row) in self.cells.iter().enumerate() {
                for x in 0..row.len() {
                    if self.is_valid_move(Vec2::new(x, y), next_stone).0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn peek_next_stone(&self) -> Option<&Stone> {
        self.stones.last()
    }
}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        for y in 0..BOARD_SIZE.1 {
            for x in 0..BOARD_SIZE.0 {
                self.draw_cell(Vec2::new(x, y), printer);
            }
        }

        let next = ("Next: ", Vec2::new(BOARD_SIZE.0 + INSET.0 * 2, INSET.1));
        printer.print(next.1, next.0);
        self.stones.last().map(|s| s.print(next.1 + (next.0.len(), 0), printer));

        let next = ("Score: ", Vec2::new(BOARD_SIZE.0 + INSET.0 * 2, INSET.1 + 2));
        printer.print(next.1, next.0);
        printer.print(next.1 + (next.0.len(), 0), &self.score.to_string());

        if self.game_over {
            printer.print(
                Vec2::new(BOARD_SIZE.0 + INSET.0 * 2, INSET.1 + 4),
                "Game over..."
            );
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(VIEW_SIZE.0, VIEW_SIZE.1)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        use cursive::event::Key::*;
        if let Event::Key(k) = event {
            let (dx, dy) = match k {
                Left => (-1, 0),
                Right => (1, 0),
                Up => (0, -1),
                Down => (0, 1),
                Enter => {
                    // TODO: handle possible failures
                    self.place_stone()
                        .map(|game_over| self.game_over = game_over);
                    return EventResult::Ignored;
                },
                _ => return EventResult::Ignored,
            };
            // TODO: use XY::saturating_add?
            self.cursor_pos.x = clamp(self.cursor_pos.x as isize + dx,
                0, BOARD_SIZE.0 as isize - 1) as usize;
            self.cursor_pos.y = clamp(self.cursor_pos.y as isize + dy,
                0, BOARD_SIZE.1 as isize - 1) as usize;
        }
        EventResult::Ignored
    }
}

#[derive(Eq, PartialEq)]
enum MoveError {
    StackEmpty,
    DoublePlace,
    InvalidMove,
}

