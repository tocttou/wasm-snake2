use std::assert;
use wasm_bindgen::prelude::*;

const DEFAULT_BOARD_WIDTH: u16 = 10;
const DEFAULT_BOARD_HEIGHT: u16 = 10;
const DEFAULT_SNAKE_CELLS: [u32; 3] = [0, 1, 2];
const DEFAULT_DIRECTION: Direction = Direction::Right;
const DEFAULT_BAIT: u32 = 8;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Lit = 1,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

pub struct Snake {
    cells: Vec<u32>,
    direction: Direction,
    head: u32,
    tail: u32,
}

#[wasm_bindgen]
pub struct Board {
    cells: Vec<Cell>,
    snake: Snake,
    bait: u32,
    width: u16,
    height: u16,
}

impl Snake {
    pub fn march(&mut self, width: u16, height: u16, cut_tail: bool) {
        self.head = match (self.direction, self.head) {
            (Direction::Up, x) => {
                let (row, col) = get_row_col(x, width);
                get_index((row + height - 1) % height, col, width)
            }
            (Direction::Right, x) => {
                let (row, col) = get_row_col(x, width);
                get_index(row, (col + 1) % width, width)
            }
            (Direction::Down, x) => {
                let (row, col) = get_row_col(x, width);
                get_index((row + 1) % height, col, width)
            }
            (Direction::Left, x) => {
                let (row, col) = get_row_col(x, width);
                get_index(row, (col + width - 1) % width, width)
            }
        };

        if cut_tail {
            self.tail = *self.cells.first().unwrap();
            self.cells.remove(0);
        }
        self.cells.push(self.head);
    }
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16) -> Board {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        assert!(
            width >= DEFAULT_BOARD_WIDTH && height >= DEFAULT_BOARD_HEIGHT,
            "width must be >= {} and height must be >= {}",
            DEFAULT_BOARD_WIDTH,
            DEFAULT_BOARD_HEIGHT
        );
        let mut cells: Vec<Cell> = (0..width * height).map(|_| Cell::Dead).collect();
        for i in DEFAULT_SNAKE_CELLS.iter().cloned() {
            cells[i as usize] = Cell::Lit;
        }

        Board {
            cells,
            width,
            height,
            bait: DEFAULT_BAIT,
            snake: Snake {
                cells: DEFAULT_SNAKE_CELLS.to_vec(),
                direction: DEFAULT_DIRECTION,
                head: DEFAULT_SNAKE_CELLS.last().unwrap().clone(),
                tail: DEFAULT_SNAKE_CELLS.first().unwrap().clone(),
            },
        }
    }

    pub fn reset(&mut self) {
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
        for i in DEFAULT_SNAKE_CELLS.iter().cloned() {
            self.cells[i as usize] = Cell::Lit;
        }
        self.bait = DEFAULT_BAIT;
        self.snake = Snake {
            cells: DEFAULT_SNAKE_CELLS.to_vec(),
            direction: DEFAULT_DIRECTION,
            head: DEFAULT_SNAKE_CELLS.last().unwrap().clone(),
            tail: DEFAULT_SNAKE_CELLS.first().unwrap().clone(),
        };
    }

    pub fn score(&self) -> usize {
        self.snake.cells.len() - DEFAULT_SNAKE_CELLS.len()
    }

    pub fn tick(&mut self) {
        self.cells[self.bait as usize] = Cell::Lit;
        self.snake.march(self.width, self.height, true);

        if self.snake.head == self.bait {
            self.snake.march(self.width, self.height, false);
            let mut bait = get_random_in_range(0, self.cells.len() - 1);
            while self.cells[bait] == Cell::Lit {
                bait = get_random_in_range(0, self.cells.len() - 1);
            }
            self.bait = bait as u32;
        }

        if self.snake.head != self.bait && self.cells[self.snake.head as usize] == Cell::Lit {
            alert("Game Over!");
            self.reset();
        }

        self.cells[self.snake.tail as usize] = Cell::Dead;
        self.cells[self.snake.head as usize] = Cell::Lit;
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
}

#[wasm_bindgen]
impl Board {
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn direction(&self) -> Direction {
        self.snake.direction
    }
}

#[wasm_bindgen]
pub fn get_memory() -> JsValue {
    wasm_bindgen::memory()
}

fn get_index(row: u16, column: u16, width: u16) -> u32 {
    (row * width + column) as u32
}

fn get_row_col(index: u32, width: u16) -> (u16, u16) {
    let row: u16 = (index / (width as u32)) as u16;
    let col: u16 = (index % (width as u32)) as u16;
    (row, col)
}

fn get_random_in_range(min: usize, max: usize) -> usize {
    (js_sys::Math::floor(js_sys::Math::random() * ((max - min + 1) as f64)) as usize) + min
}
