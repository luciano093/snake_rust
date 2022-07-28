use std::rc::Rc;
use std::cell::RefCell;

use crate::{GRID_ROWS, GRID_COLS};
use crate::Position;

use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

pub struct Apple {
    grid: Rc<RefCell<[[u8; GRID_COLS]; GRID_ROWS]>>,
    position: Position,

    range: ThreadRng,
    rand_row: Uniform<i32>,
    rand_col: Uniform<i32>,
}

impl Apple {
    pub fn new(grid: Rc<RefCell<[[u8; GRID_COLS]; GRID_ROWS]>>) -> Self {
        let range: ThreadRng = rand::thread_rng();
        let rand_row: Uniform<i32> = Uniform::from(0..GRID_ROWS as i32);
        let rand_col: Uniform<i32> = Uniform::from(0..GRID_COLS as i32);

        let position = Position { x: GRID_COLS as i32 / 2, y: GRID_ROWS as i32 / 2 };

        grid.as_ref().borrow_mut()[position.y as usize][position.x as usize] = 2;

        Apple { grid, position, range, rand_row, rand_col }
    }

    pub fn rand_pos(&mut self) {
        let (mut new_x, mut new_y) = (self.rand_col.sample(&mut self.range), self.rand_row.sample(&mut self.range));

        while self.grid.as_ref().borrow()[new_y as usize][new_x as usize] != 0 {
            (new_x, new_y) = (self.rand_col.sample(&mut self.range), self.rand_row.sample(&mut self.range));
        }

        self.position.x = new_x;
        self.position.y = new_y;

        self.grid.as_ref().borrow_mut()[self.position.y as usize][self.position.x as usize] = 2;
    }

    pub fn get_pos(&self) -> &Position {
        &self.position
    }
}