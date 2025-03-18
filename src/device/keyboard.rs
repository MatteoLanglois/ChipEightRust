use crate::exceptions::{Exception};
use crate::exceptions::ExceptionType::SDL;

pub struct Keyboard {
    map: [u8; 16]
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { map: [0; 16] }
    }

    pub fn get(&self, key: u8) -> Option<u8> {
        if key < 16 {
            Some(self.map[key as usize])
        } else {
            None
        }
    }

    pub fn wait(&self, key: u8) -> Result<u8, Exception> {
        if key < 16 {
            Ok(self.map[key as usize])
        } else {
            Err(Exception::new(SDL))
        }
    }
}