use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::exceptions::{Exception};
use crate::exceptions::ExceptionType::SDL;

pub struct Keyboard {
    map: [Keycode; 16],
    pressed_keys: [u8; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let key_map = [
            Keycode::X, Keycode::Num1, Keycode::Num2,
            Keycode::Num3, Keycode::A, Keycode::Z,
            Keycode::E, Keycode::Q, Keycode::S,
            Keycode::D, Keycode::W, Keycode::C,
            Keycode::Num4, Keycode::R, Keycode::F, Keycode::V
        ];
        Keyboard { map: key_map, pressed_keys: [0; 16] }
    }

    pub fn get(&self, key: u8) -> Option<u8> {
        if key >= 16 {
            None
        } else {
            Some(self.pressed_keys[key as usize])
        }
    }

    pub fn wait(&self, key: u8) -> Result<u8, Exception> {
        if key >= 16 {
            return Err(Exception::new(SDL));
        }
        for (i, &k) in self.map.iter().enumerate() {
            if k == self.map[key as usize] {
                return Ok(i as u8);
            }
        }
        Ok(key)
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown { keycode: Some(keycode), .. } => {
                if let Some(chip8_key) = self.map.iter().position(|&k| k == keycode) {
                    self.pressed_keys[chip8_key] = 1;
                }
            }
            Event::KeyUp { keycode: Some(keycode), .. } => {
                if let Some(chip8_key) = self.map.iter().position(|&k| k == keycode) {
                    self.pressed_keys[chip8_key] = 0;
                }
            }
            _ => {}
        }
    }
}