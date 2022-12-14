use std::mem::size_of;

use sdl2::Sdl;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};

pub mod snake;
pub mod apple;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 400;
pub const GRID_ROWS: usize = 10;
pub const GRID_COLS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn init_window(title: &str, width: u32, height: u32) -> (Canvas<Window>, Sdl) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, width, height).position_centered().build().unwrap();

    let canvas = window.into_canvas().build().unwrap();

    (canvas, sdl_context)
}

pub fn create_texture(texture_creator: &TextureCreator<WindowContext>, size: u32, color: Color) -> Texture {
    let mut data = vec![0; size as usize * size as usize * 3];

    let mut state = 0i8;
    data.iter_mut().for_each(|num| {
       if state == 0 {
        *num = color.b;
       }
       else if state == 1 {
        *num = color.g;
       }
       else if state == 2 {
        *num = color.r;
       }
       else if state == 3 {
        *num = color.a;
        state = -1;
       }

       state += 1;
    });

    let surface = Surface::from_data(&mut data, size, size, size * size_of::<u8>() as u32, PixelFormatEnum::RGB888).unwrap();
    
    Texture::from_surface(&surface, texture_creator).unwrap()
}

pub fn get_grid_size(grid: &[[u8; GRID_COLS]; GRID_ROWS]) -> (u32, u32) {
    let cell_width = WIDTH / grid.len() as u32;
    let cell_height = HEIGHT / grid[0].len() as u32;

    (cell_width, cell_height)
}