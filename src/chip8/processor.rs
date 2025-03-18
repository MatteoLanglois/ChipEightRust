use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::chip8::memory::RandomAccessMemory;
use crate::device::display::Display;
use crate::device::keyboard::Keyboard;
use crate::device::sprite::Sprite;
use crate::exceptions::{Exception, ExceptionType};

pub struct Processor {
    reg_v: [u8; 16],
    i: u16,
    pub(crate) dt: u8,
    pub(crate) st: u8,

    program_counter: u16,
    stack: [u16; 16],
    stack_ptr: u16,

    memory: Rc<RefCell<RandomAccessMemory>>,
    display: Rc<RefCell<Display>>,
    keyboard: Rc<RefCell<Keyboard>>,
}

impl Processor {
    pub fn new(ram: Rc<RefCell<RandomAccessMemory>>, display: Rc<RefCell<Display>>,
               keyboard: Rc<RefCell<Keyboard>>) -> Processor {
       Processor {
           reg_v: [0; 16],
           i: 0,
           dt: 0,
           st: 0,
           program_counter: 512,
           stack: [0; 16],
           stack_ptr: 0,

           memory: ram,
           display,
           keyboard,
       }
    }

    pub fn fetch_decode_execute(&mut self) -> Result<(), Exception> {
        let part1 = self.memory.borrow().read(self.program_counter)?;
        let part2 = self.memory.borrow().read(self.program_counter + 1)?;
        let mut instr: u16 = (part1 as u16) << 8;
        instr = instr + part2 as u16;

        // println!("Instr : {:X} - {}", instr, self.program_counter);

        self.program_counter += 2;

        if instr == 0x00E0 {
            self.processor_00e0_cls()
        } else if instr == 0x00EE {
            self.processor_00ee_ret()
        } else if (instr & 0xF000) == 0 {
            self.processor_0nnn_sys(instr)
        } else if (instr & 0xF000) == 0x1000 {
            self.processor_1nnn_jpt(instr & 0x0FFF)
        } else if (instr & 0xF000) == 0x2000 {
            self.processor_2nnn_call(instr & 0x0FFF)
        } else if (instr & 0xF000) == 0x3000 {
            self.processor_3xkk_se(((instr & 0x0F00) >> 8) as u8, (instr & 0x00FF) as u8)
        } else if (instr & 0xF000) == 0x4000 {
            self.processor_4xkk_sne(((instr & 0x0F00) >> 8) as u8, (instr & 0x00FF) as u8)
        } else if (instr & 0xF000) == 0x5000 {
            self.processor_5xy0_sereg(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF000) == 0x6000 {
            self.processor_6xkk_ldval(((instr & 0x0F00) >> 8) as u8, (instr & 0x00FF) as u8)
        } else if (instr & 0xF000) == 0x7000 {
            self.processor_7xkk_add(((instr & 0x0F00) >> 8) as u8, (instr & 0x00FF) as u8)
        } else if (instr & 0xF00F) == 0x8000 {
            self.processor_8xy0_ldreg(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x8001 {
            self.processor_8xy1_or(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x8002 {
            self.processor_8xy2_and(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x8003 {
            self.processor_8xy3_xor(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x8004 {
            self.processor_8xy4_addc(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x8005 {
            self.processor_8xy5_sub(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x8006 {
            self.processor_8xy6_shr(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x8007 {
            self.processor_8xy7_subn(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF00F) == 0x800E {
            self.processor_8xye_shl(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF000) == 0x9000 {
            self.processor_9xy0_sne_reg(((instr & 0x0F00) >> 8) as u8, ((instr & 0x00F0) >> 4) as u8)
        } else if (instr & 0xF000) == 0xA000 {
            self.processor_annn_ldi(instr & 0x0FFF)
        } else if (instr & 0xF000) == 0xB000 {
            self.processor_bnnn_jpv0(instr & 0x0FFF)
        } else if (instr & 0xF000) == 0xC000 {
            self.processor_cxkk_rnd(((instr & 0x0F00) >> 8) as u8, (instr & 0x00FF) as u8)
        } else if (instr & 0xF000) == 0xD000 {
            self.processor_dxyn_drw(((instr & 0x0F00) >> 8) as u8,
                                    ((instr & 0x00F0) >> 4) as u8,
                                    (instr & 0x000F) as u8)
        } else if (instr & 0xF0FF) == 0xE09E {
            self.processor_ex9e_skp(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xE0A1 {
            self.processor_exa1_sknp(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF007 {
            self.processor_fx07_lddt(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF00A {
            self.processor_fx0a_ldvk(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF015 {
            self.processor_fx15_lddt(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF018 {
            self.processor_fx18_ldst(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF01E {
            self.processor_fx1e_addi(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF029 {
            self.processor_fx29_ldf(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF033 {
            self.processor_fx33_ldb(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF055 {
            self.processor_fx55_ldw(((instr & 0x0F00) >> 8) as u8)
        } else if (instr & 0xF0FF) == 0xF065 {
            self.processor_fx65_ldr(((instr & 0x0F00) >> 8) as u8)
        } else {
            Err(Exception::new(ExceptionType::BadInstruction))
        }
    }

    pub(crate) fn load_sprites(&mut self) -> Result<(), Exception> {
        let sprite_list: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
        ];

        for (i, sprite) in sprite_list.iter().enumerate() {
            for (j, &byte) in sprite.iter().enumerate() {
                self.memory.borrow_mut().write((431 + i * 5 + j) as u16, byte)?;
            }
        }

        Ok(())
    }

    fn processor_0nnn_sys(&mut self, address: u16) -> Result<(), Exception> {
        if address <= 4095 && address >= 512 {
            self.program_counter = address;
            Ok(())
        } else {
            Err(Exception::new(ExceptionType::AddressOutOfRange))
        }
    }

    fn processor_00e0_cls(&mut self) -> Result<(), Exception> {
        match self.display.borrow_mut().clear() {
            Ok(_) => Ok(()),
            Err(_) => Err(Exception::new(ExceptionType::SDL))
        }
    }

    fn processor_00ee_ret(&mut self) -> Result<(), Exception> {
        if self.stack_ptr == 0 {
            Err(Exception::new(ExceptionType::StackPointerOutOfRange))
        } else {
            self.program_counter = self.stack[self.stack_ptr as usize];
            self.stack_ptr -= 1;
            Ok(())
        }
    }

    fn processor_1nnn_jpt(&mut self, address: u16) -> Result<(), Exception> {
        if address <= 4095 {
            self.program_counter = address;
            Ok(())
        } else {
            Err(Exception::new(ExceptionType::AddressOutOfRange))
        }
    }

    fn processor_2nnn_call(&mut self, address: u16) -> Result<(), Exception> {
        if self.stack_ptr > 15 {
            Err(Exception::new(ExceptionType::StackPointerOutOfRange))
        } else {
            self.stack_ptr += 1;
            self.stack[self.stack_ptr as usize] = self.program_counter;
            self.program_counter = address;
            Ok(())
        }
    }

    fn processor_3xkk_se(&mut self, reg: u8, val: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        if self.reg_v[reg as usize] == val {
            self.program_counter += 2;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn processor_4xkk_sne(&mut self, reg: u8, val: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        if self.reg_v[reg as usize] != val {
            self.program_counter += 2;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn processor_5xy0_sereg(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        if self.reg_v[reg1 as usize] == self.reg_v[reg2 as usize] {
            self.program_counter += 2;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn processor_6xkk_ldval(&mut self, reg: u8, val: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.reg_v[reg as usize] = val;
        Ok(())
    }

    fn processor_7xkk_add(&mut self, reg: u8, val: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        let sum = self.reg_v[reg as usize] as u16 + val as u16;
        self.reg_v[reg as usize] = sum as u8;
        Ok(())
    }

    fn processor_8xy0_ldreg(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.reg_v[reg1 as usize] = self.reg_v[reg2 as usize];
        Ok(())
    }

    fn processor_8xy1_or(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.reg_v[reg1 as usize] = self.reg_v[reg1 as usize] | self.reg_v[reg2 as usize];
        self.reg_v[15] = 0;
        Ok(())
    }

    fn processor_8xy2_and(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.reg_v[reg1 as usize] = self.reg_v[reg1 as usize] & self.reg_v[reg2 as usize];
        self.reg_v[15] = 0;
        Ok(())
    }

    fn processor_8xy3_xor(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.reg_v[reg1 as usize] = self.reg_v[reg1 as usize] ^ self.reg_v[reg2 as usize];
        self.reg_v[15] = 0;
        Ok(())
    }

    fn processor_8xy4_addc(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        let sum = self.reg_v[reg1 as usize] as u16 + self.reg_v[reg2 as usize] as u16;
        self.reg_v[reg1 as usize] = sum as u8;
        if sum > 255 {
            self.reg_v[15] = 1;
            Ok(())
        } else {
            self.reg_v[15] = 0;
            Ok(())
        }
    }

    fn processor_8xy5_sub(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        if self.reg_v[reg1 as usize] >= self.reg_v[reg2 as usize] {
            self.reg_v[reg1 as usize] -= self.reg_v[reg2 as usize];
            self.reg_v[15] = 1;
            Ok(())
        } else {
            let sub: i16 = self.reg_v[reg1 as usize] as i16 - self.reg_v[reg2 as usize] as i16;
            if sub < 0 {
                self.reg_v[reg1 as usize] = (sub + 256) as u8;
            } else {
                self.reg_v[reg1 as usize] = sub as u8;
            }
            self.reg_v[15] = 0;
            Ok(())
        }
    }

    fn processor_8xy6_shr(&mut self, reg1: u8, _reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument));
        }
        self.reg_v[15] = self.reg_v[reg1 as usize] & 0x1;
        self.reg_v[reg1 as usize] >>= 1;
        Ok(())
    }

    fn processor_8xy7_subn(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        if self.reg_v[reg2 as usize] >= self.reg_v[reg1 as usize] {
            self.reg_v[reg1 as usize] = self.reg_v[reg2 as usize] - self.reg_v[reg1 as usize];
            self.reg_v[15] = 1;
            Ok(())
        } else {
            println!("Reg1 : {} - Reg2 : {}", self.reg_v[reg1 as usize], self.reg_v[reg2 as usize]);
            self.reg_v[reg1 as usize] = self.reg_v[reg1 as usize] - self.reg_v[reg2 as usize];
            self.reg_v[15] = 0;
            Ok(())
        }
    }

    fn processor_8xye_shl(&mut self, reg1: u8, _reg2: u8) -> Result<(), Exception> {
        if reg1 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument));
        }
        self.reg_v[15] = (self.reg_v[reg1 as usize] & 0x80) >> 7;
        self.reg_v[reg1 as usize] <<= 1;
        Ok(())
    }

    fn processor_9xy0_sne_reg(&mut self, reg1: u8, reg2: u8) -> Result<(), Exception> {
        if self.reg_v[reg1 as usize] != self.reg_v[reg2 as usize] {
            self.program_counter += 2;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn processor_annn_ldi(&mut self, address: u16) -> Result<(), Exception>  {
        if address <= 4095 {
            self.i = address;
            Ok(())
        } else {
            Err(Exception::new(ExceptionType::AddressOutOfRange))
        }
    }

    fn processor_bnnn_jpv0(&mut self, address: u16) -> Result<(), Exception>  {
        if address <= 4095 {
            self.program_counter = address + self.reg_v[0] as u16;
            Ok(())
        } else {
            Err(Exception::new(ExceptionType::AddressOutOfRange))
        }
    }

    fn processor_cxkk_rnd(&mut self, reg1: u8, val: u8) -> Result<(), Exception> {
        if reg1 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        let mut rng = rand::rng();
        let random: u8 = rng.random();
        let y = random / 255;
        self.reg_v[reg1 as usize] = y & val;
        Ok(())
    }

    fn processor_dxyn_drw(&mut self, reg1: u8, reg2: u8, nibble: u8) -> Result<(), Exception> {
        if reg1 > 15 || reg2 > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        let mut sprite_content: Vec<u8> = Vec::with_capacity(nibble as usize);

        for i in self.i..self.i + nibble as u16 {
            if i >= 4096 {
                return Err(Exception::new(ExceptionType::SDL))
            } else {
                let byte = self.memory.borrow_mut().read(i)?;
                sprite_content.push(byte);
            }
        }

        let sprite = Sprite::new_with_content(sprite_content);

        self.display.borrow_mut().draw(&sprite, self.reg_v[reg1 as usize], self.reg_v[reg2 as usize], &mut self.reg_v[15])?;
        Ok(())
    }

    fn processor_ex9e_skp(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        if self.keyboard.borrow_mut().get(self.reg_v[reg as usize]) == Some(1) {
            self.program_counter += 2;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn processor_exa1_sknp(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        if self.keyboard.borrow_mut().get(self.reg_v[reg as usize]) == Some(0) {
            self.program_counter += 2;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn processor_fx07_lddt(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.reg_v[reg as usize] = self.dt;
        Ok(())
    }

    fn processor_fx0a_ldvk(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.keyboard.borrow_mut().wait(self.reg_v[reg as usize])?;
        Ok(())
    }

    fn processor_fx15_lddt(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.dt = self.reg_v[reg as usize];
        Ok(())
    }

    fn processor_fx18_ldst(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.st = self.reg_v[reg as usize];
        Ok(())
    }

    fn processor_fx1e_addi(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.i += self.reg_v[reg as usize] as u16;
        Ok(())
    }

    fn processor_fx29_ldf(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        self.i = 431 + reg as u16 * 5;
        Ok(())
    }

    fn processor_fx33_ldb(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        let value: u8 = self.reg_v[reg as usize];

        self.memory.borrow_mut().write(self.i, value / 100)?;
        self.memory.borrow_mut().write(self.i + 1, (value / 10) % 10)?;
        self.memory.borrow_mut().write(self.i + 2, value % 10)?;
        Ok(())
    }

    fn processor_fx55_ldw(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        for i in 0..=reg {
            self.memory.borrow_mut().write(self.i + i as u16, self.reg_v[i as usize])?;
        }
        Ok(())
    }

    fn processor_fx65_ldr(&mut self, reg: u8) -> Result<(), Exception> {
        if reg > 15 {
            return Err(Exception::new(ExceptionType::BadArgument))
        }
        for i in 0..=reg {
            self.reg_v[i as usize] = self.memory.borrow_mut().read(self.i + i as u16)?;
        }
        Ok(())
    }
}