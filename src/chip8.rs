use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::time::{Duration, Instant};
use crate::chip8::memory::RandomAccessMemory;
use crate::chip8::processor::Processor;
use crate::device::display::Display;
use crate::device::keyboard::Keyboard;
use crate::device::speaker::Speaker;
use crate::exceptions::Exception;
use crate::exceptions::ExceptionType::{SDL};

mod memory;
mod processor;

pub struct Chip8 {
    processor: Processor,
    ram: Rc<RefCell<RandomAccessMemory>>,
    display: Rc<RefCell<Display>>,
    speaker: Rc<RefCell<Speaker>>,
    keyboard: Rc<RefCell<Keyboard>>,

    sdl_context: sdl2::Sdl,
}

impl Chip8 {
    pub fn new(rom_content: &str) -> Chip8 {
        let sdl_context = sdl2::init().map_err(|_| Exception::new(SDL)).unwrap();
        let ram = Rc::new(RefCell::new(RandomAccessMemory::new()));
        let display = Rc::new(RefCell::new(Display::new(&sdl_context.video().unwrap(), sdl_context.timer().unwrap())));
        let keyboard = Rc::new(RefCell::new(Keyboard::new()));
        let speaker = Rc::new(RefCell::new(Speaker::new(&sdl_context.audio().unwrap())));

        let mut processor = Processor::new(
            Rc::clone(&ram),
            Rc::clone(&display),
            Rc::clone(&keyboard));
        processor.load_sprites().unwrap();

        let mut c8 = Chip8 {
            processor,
            ram,
            display,
            speaker,
            keyboard,
            sdl_context,
        };
        c8.load_rom(&*c8.read_rom(rom_content).unwrap()).expect("Can't read ROM");

        c8
    }

    pub(crate) fn start(&mut self) -> Result<(), Exception> {
        self.cycle()?;
        Ok(())
    }

    fn read_rom(&self, rom_path: &str) -> Result<Vec<u8>, Exception> {
        fs::read(rom_path).map_err(|_| Exception::new(SDL))
    }

    fn load_rom(&mut self, rom_content: &[u8]) -> Result<(), Exception> {
        // Write the file content to the memory
        let mut address = 512;
        for &byte in rom_content{
            self.ram.borrow_mut().write(address, byte).expect("Can't write to memory");
            address += 1;
        }

        Ok(())

    }

    fn cycle(&mut self) -> Result<(), Exception> {
        let mut event_pump = self.sdl_context.event_pump().map_err(|_| Exception::new(SDL))?;
        let mut cpt = 0;
        let frame_time = Duration::from_millis(16);
        let mut last_time = Instant::now();

        loop {
            for event in event_pump.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    println!("Quitting");
                    return Ok(());
                }
            }

            self.processor.fetch_decode_execute()?;

            if cpt % 2 == 0 {
                self.display.borrow_mut().update()?;
            }

            let elapsed = last_time.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
            last_time = Instant::now();
            cpt += 1;
        }
    }
}
