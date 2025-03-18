extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::{TimerSubsystem, VideoSubsystem};
use sdl2::video::Window;
use crate::device::sprite::Sprite;
use crate::exceptions::Exception;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 960;
const SCALE: u32 = 30;

pub struct Display {
    pixel: i16,
    content: [[i16; 64]; 32],
    canvas: Canvas<Window>,
    timer: TimerSubsystem,
    modified: i8,
}

impl Display {
    pub fn new(video_subsystem: &VideoSubsystem, timer: TimerSubsystem) -> Self {
        let window = video_subsystem.window("Chip8", WIDTH, HEIGHT)
            .position_centered()
            .build().unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display {
            pixel: 30,
            content: [[0; 64]; 32],
            canvas,
            timer,
            modified: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), Exception> {
        if self.modified == 0 {
            return Ok(());
        }
        for y in 0..32 {
            for x in 0..64 {
                self.draw_pixel(x as u8, y as u8, self.content[y][x] as u8);
            }
        }
        self.canvas.present();
        self.timer.delay(1000 / 60);
        self.modified = 0;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Exception> {
        self.canvas.clear();
        for y in 0..32 {
            for x in 0..64 {
                self.content[y][x] = 0;
            }
        }
        self.update()
    }

    pub fn draw(&mut self, sprite: &Sprite, x: u8, y: u8, vf: &mut u8) -> Result<(), Exception> {
        let x_pos: u32 = x as u32 % (WIDTH / SCALE);
        let y_pos: u32 = y as u32 % (HEIGHT / SCALE);

        println!("x_pos: {}, y_pos: {}", x_pos, y_pos);

        *vf = 0;

        for line_count in 0..sprite.length() {
            for column_count in 0..8 {
                if x_pos + column_count < WIDTH / SCALE &&
                    y_pos + (line_count as u32) < HEIGHT / SCALE {
                    let pixel_value: bool = match sprite.get(line_count) {
                        Ok(value) => value & (0x80 >> column_count) != 0,
                        Err(e) => return Err(e),
                    };
                    let x_pos_new: usize = ((x_pos + column_count) % (WIDTH / SCALE)) as usize;
                    let y_pos_new: usize = ((y_pos + line_count as u32) % (HEIGHT / SCALE)) as usize;
                    let old_pixel: bool = self.content[y_pos_new][x_pos_new] == 1;

                    if pixel_value && old_pixel {
                        self.content[y_pos_new][x_pos_new] = 0;
                        *vf = 1;
                        self.modified = 1;
                    } else {
                        self.content[y_pos_new][x_pos_new] = 1;
                        self.modified = 1;
                    }
                }
            }
        }
        self.update()
    }

    fn draw_pixel(&mut self, x: u8, y: u8, value: u8) {
        let color = if value == 0 { 0 } else { 255 };
        let pixel: Rect = Rect::new(
            x as i32 * self.pixel as i32,
            y as i32 * self.pixel as i32,
            self.pixel as u32,
            self.pixel as u32,
        );
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, color, 0));
        self.canvas.fill_rect(pixel).unwrap();
    }
}