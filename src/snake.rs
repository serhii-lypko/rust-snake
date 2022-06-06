use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>, // TODO: try to redo everything with a vector
}

impl Snake {
    // TODO: take random x and y
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            // tail: None,
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, g)
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();

        (head_block.x, head_block.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        self.grow();

        self.body.pop_back().unwrap();

        // let removed_block = self.body.pop_back().unwrap();
        // self.tail = Some(removed_block);
    }

    pub fn grow(&mut self) {
        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block);
    }

    pub fn is_overlap(&self) -> bool {
        let (head_x, head_y): (i32, i32) = self.head_position();
        let tail = self.get_tail();

        for block in tail {
            if head_x == block.x && head_y == block.y {
                return true;
            }
        }

        false
    }

    /* Getting body without head */
    fn get_tail(&self) -> LinkedList<Block> {
        // TODO: find better way?
        let mut without_head = self.body.clone();
        without_head.pop_front().unwrap();

        without_head
    }

    // pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
    //     let (head_x, head_y): (i32, i32) = self.head_position();

    //     let mut moving_dir = self.direction;
    //     match dir {
    //         Some(d) => moving_dir = d,
    //         None => {}
    //     }

    //     match moving_dir {
    //         Direction::Up => (head_x, head_y - 1),
    //         Direction::Down => (head_x, head_y + 1),
    //         Direction::Left => (head_x - 1, head_y),
    //         Direction::Right => (head_x + 1, head_y),
    //     }
    // }
}
