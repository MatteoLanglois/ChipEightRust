mod chip8;
mod exceptions;
mod device;

fn main() {
    //let mut c8 = chip8::Chip8::new("roms/1-chip8-logo.ch8");
    //let mut c8 = chip8::Chip8::new("roms/3-corax+.ch8");
    let mut c8 = chip8::Chip8::new("roms/4-flags.ch8");
    //let mut c8 = chip8::Chip8::new("roms/5-quirks.ch8");
    //let mut c8 = chip8::Chip8::new("roms/IBM_Logo.ch8");
    c8.start().expect("Chip8 crashed");
}