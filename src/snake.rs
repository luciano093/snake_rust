use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::{GRID_ROWS, GRID_COLS};
use crate::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left, Right, Up, Down
}

pub struct Snake {
    grid: Rc<RefCell<[[u8; GRID_COLS]; GRID_ROWS]>>,
    parts: VecDeque<Position>,
    dead: bool,

    pub direction: Direction,
}

impl Snake {
    pub fn new(grid: Rc<RefCell<[[u8; GRID_COLS]; GRID_ROWS]>>) -> Self {
        let parts = VecDeque::from([Position{ x: 0, y: GRID_ROWS as i32 / 2 }]);
        (*grid.borrow_mut())[parts[0].y as usize][parts[0].x as usize] = 1;

        Snake { grid, parts, direction: Direction::Right, dead: false }
    }

    pub fn r#move(&mut self) {
        match self.direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        }
    }
    
    pub fn grow(&mut self) {
        self.parts.push_back(self.get_tail_pos().clone());
    }

    pub fn get_pos(&self) -> &Position {
        self.parts.front().unwrap()
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    fn get_tail_pos(&self) -> &Position {
        self.parts.back().unwrap()
    }

    fn move_left(&mut self) {
        if self.get_pos().x - 1 < 0 {
            self.dead = true;
            return;
        }

        if self.grid.borrow()[self.get_pos().y as usize][self.get_pos().x as usize - 1] == 1 {
            self.dead = true;
            return;
        } 

        self.parts.push_front(Position { x: self.get_pos().x - 1, y: self.get_pos().y });

        (*self.grid.borrow_mut())[self.get_pos().y as usize][self.get_pos().x as usize] = 1;
        (*self.grid.borrow_mut())[self.get_tail_pos().y as usize][self.get_tail_pos().x as usize] = 0;

        self.parts.pop_back();
    }

    fn move_right(&mut self) {
        if self.get_pos().x + 1 >= GRID_COLS as i32 {
            self.dead = true;
            return;
        }

        if self.grid.borrow()[self.get_pos().y as usize][self.get_pos().x as usize + 1] == 1 {
            self.dead = true;
            return;
        } 

        self.parts.push_front(Position { x: self.get_pos().x + 1, y: self.get_pos().y });

        (*self.grid.borrow_mut())[self.get_pos().y as usize][self.get_pos().x as usize] = 1;
        (*self.grid.borrow_mut())[self.get_tail_pos().y as usize][self.get_tail_pos().x as usize] = 0;

        self.parts.pop_back();
    }

    fn move_up(&mut self) {
        if self.get_pos().y - 1 < 0 {
            self.dead = true;
            return;
        }

        if self.grid.borrow()[self.get_pos().y as usize - 1][self.get_pos().x as usize] == 1 {
            self.dead = true;
            return;
        } 

        self.parts.push_front(Position { x: self.get_pos().x, y: self.get_pos().y - 1 });

        (*self.grid.borrow_mut())[self.get_pos().y as usize][self.get_pos().x as usize] = 1;
        (*self.grid.borrow_mut())[self.get_tail_pos().y as usize][self.get_tail_pos().x as usize] = 0;

        self.parts.pop_back();
    }

    fn move_down(&mut self) {
        if self.get_pos().y + 1 >= GRID_ROWS as i32 {
            self.dead = true;
            return;
        }

        if self.grid.borrow()[self.get_pos().y as usize + 1][self.get_pos().x as usize] == 1 {
            self.dead = true;
            return;
        } 

        self.parts.push_front(Position { x: self.get_pos().x, y: self.get_pos().y + 1 });

        (*self.grid.borrow_mut())[self.get_pos().y as usize][self.get_pos().x as usize] = 1;
        (*self.grid.borrow_mut())[self.get_tail_pos().y as usize][self.get_tail_pos().x as usize] = 0;

        self.parts.pop_back();
    }
}