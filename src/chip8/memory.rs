use crate::exceptions::Exception;
use crate::exceptions::ExceptionType::AddressOutOfRange;

const RAM_MAX: usize = 4096;

pub struct RandomAccessMemory {
    memory: [u8; RAM_MAX]
}

impl RandomAccessMemory {
    pub(crate) fn new() -> RandomAccessMemory {
        RandomAccessMemory {
            memory: [0; RAM_MAX]
        }
    }

    pub(crate) fn read(&self, address: u16) -> Result<u8, Exception> {
        if address < RAM_MAX as u16 {
            Ok(self.memory[address as usize])
        } else {
            Err(Exception::new(AddressOutOfRange))
        }
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) -> Result<(), Exception> {
        if address < RAM_MAX as u16 {
            self.memory[address as usize] = value;
            Ok(())
        } else {
            Err(Exception::new(AddressOutOfRange))
        }
    }
}