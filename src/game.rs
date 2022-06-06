use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::draw_block;
use crate::snake::{Direction, Snake};
use crate::utils::generate_random_coordinates;

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];

const INITIAL_MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0;

#[derive(Debug)]
pub struct Food {
    x: i32,
    y: i32,
}

impl Food {
    pub fn new(x: i32, y: i32) -> Self {
        Food { x, y }
    }

    pub fn regenerate(&mut self, canvas_width: i32, canvas_height: i32) {
        let (updated_x, updated_y) = generate_random_coordinates(canvas_width, canvas_height);

        self.x = updated_x;
        self.y = updated_y;
    }
}

pub struct Game {
    snake: Snake,
    food: Food,
    score: i32,
    waiting_time: f64,
    moving_period: f64,
    width: i32,
    height: i32,
    game_over: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Game {
            snake: Snake::new(2, 2),
            food: Food::new(4, 4),
            score: 0,
            waiting_time: 0.0,
            moving_period: INITIAL_MOVING_PERIOD,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);
        self.draw_food(ctx, g);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.waiting_time > self.moving_period {
            self.update_snake(None);
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food = Food::new(4, 4);
        self.score = 0;
    }

    fn draw_food(&self, ctx: &Context, g: &mut G2d) {
        draw_block(FOOD_COLOR, self.food.x, self.food.y, ctx, g);
    }

    fn validate_move(&mut self) {
        let (head_x, head_y) = self.snake.head_position();

        let x_is_invalid = head_x > self.width || head_x < 0;
        let y_is_invalid = head_y > self.height || head_y < 0;
        let is_overlap = self.snake.is_overlap();

        if x_is_invalid || y_is_invalid || is_overlap {
            self.restart();
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        self.snake.move_forward(dir);

        let (head_x, head_y) = self.snake.head_position();

        if head_x == self.food.x && head_y == self.food.y {
            self.food.regenerate(self.width, self.height);
            self.snake.grow();
            self.score = self.score + 1;

            // TODO: decreasing coeficient should be dynamic
            // self.moving_period = self.moving_period - 0.1;
        }

        self.validate_move();

        self.waiting_time = 0.0;
    }

    // fn generate_random_coordinates(&self) -> (i32, i32) {
    //     let random_x = thread_rng().gen_range(0..self.width);
    //     let random_y = thread_rng().gen_range(0..self.height);
    //     (random_x, random_y)
    // }
}
