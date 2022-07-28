use std::cell::RefCell;
use std::rc::Rc;

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use snake_2::apple::Apple;
use snake_2::{GRID_ROWS, GRID_COLS};
use snake_2::{WIDTH, HEIGHT};

use snake_2::init_window;
use snake_2::create_texture;
use snake_2::get_grid_sizes;

use snake_2::snake::{Snake, Direction};

fn handle_events(event_pump: &mut EventPump) -> Option<Keycode> {
    for event in event_pump.poll_iter() {
        match event { 
            Event::Quit {..} => std::process::exit(0),
            Event::KeyDown { keycode: Some(keycode), .. } => return Some(keycode),
            _ => (),
        }
    };

    None
}

fn main() {
    let (mut canvas, sdl_context) = init_window("Snake", WIDTH, HEIGHT);
    canvas.set_draw_color(Color::WHITE);

    let grid = [[0u8; GRID_COLS]; GRID_ROWS];

    assert!(WIDTH % grid.len() as u32 == 0 && HEIGHT % grid[0].len() as u32 == 0);

    let (cell_width, cell_height) = get_grid_sizes(&grid);

    let texture_creator = canvas.texture_creator();
    let texture = create_texture(&texture_creator, cell_width, 255, 9);
    let apple_texture = create_texture(&texture_creator, cell_height, 255, 2);

    let grid = Rc::new(RefCell::new(grid));
    let mut snake = Snake::new(grid.clone());
    let mut apple = Apple::new(grid.clone());

    let mut delay = 0;
    let mut max_delay = 420;

    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        canvas.clear();
        delay += 1;

        match handle_events(&mut event_pump) {
            Some(Keycode::Left) if snake.direction != Direction::Right => snake.direction = Direction::Left,
            Some(Keycode::Right) if snake.direction != Direction::Left => snake.direction = Direction::Right,
            Some(Keycode::Up) if snake.direction != Direction::Down => snake.direction = Direction::Up,
            Some(Keycode::Down) if snake.direction != Direction::Up => snake.direction = Direction::Down,
            Some(Keycode::Plus) | Some(Keycode::Equals) if max_delay > 0 => max_delay -= 20,
            Some(Keycode::Minus) => max_delay += 20,
            _ => (),
        };

        grid.as_ref().borrow().iter().enumerate().for_each(|(row, arr)| {
            arr.iter().enumerate().for_each(|(col, &coord)| {
                if coord == 1 {
                    let rect = Rect::new(col as i32 * cell_width as i32, row as i32 * cell_height as i32, cell_width, cell_height);
                    canvas.copy(&texture, None, rect).unwrap();
                }
                else if coord == 2 {
                    let rect = Rect::new(col as i32 * cell_width as i32, row as i32 * cell_height as i32, cell_width, cell_height);
                    canvas.copy(&apple_texture, None, rect).unwrap();
                }
            });
        });

        if delay > max_delay {
            delay = 0;

            snake.r#move();

            if snake.is_dead() {
                println!("snake died :(");
                break;
            }

            if snake.get_pos() == apple.get_pos() {
                snake.grow();
                apple.rand_pos();
            }
        }

        canvas.present();
    }
}
