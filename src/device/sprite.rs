use crate::exceptions::Exception;
use crate::exceptions::ExceptionType::SDL;

#[derive(Debug)]
pub struct Sprite {
    length: usize,
    cap: usize,
    contents: Vec<u8>,
}

impl Sprite {
    pub fn new(cap: usize) -> Sprite {
        Sprite {
            length: 0,
            cap,
            contents: Vec::new(),
        }
    }

    pub(crate) fn new_with_content(p0: Vec<u8>) -> Sprite {
        Sprite {
            length: p0.len(),
            cap: p0.len(),
            contents: p0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn get(&self, i: usize) -> Result<u8, Exception> {
        if i < self.length {
            Ok(self.contents[i])
        } else {
            Err(Exception::new(SDL))
        }
    }

    pub fn add(&mut self, value: u8) -> Result<(), Exception> {
        if self.length < self.cap {
            self.length += 1;
            self.contents.push(value);
            Ok(())
        } else {
            Err(Exception::new(SDL))
        }
    }

    pub fn set(&mut self, values: Vec<u8>) -> Result<(), Exception> {
        self.contents.clear();
        self.length = 0;
        for value in values.iter() {
            self.add(*value)?;
        }
        Ok(())
    }
}