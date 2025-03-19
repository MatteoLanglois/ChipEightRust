use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, Channel, Chunk};

pub struct Speaker {
}

impl Speaker {
    pub(crate) fn new() -> Speaker {
        let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)
            .map_err(|e| e.to_string()).unwrap();
        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)
            .map_err(|e| e.to_string()).unwrap();
        sdl2::mixer::allocate_channels(4);

        Speaker { }
    }

    pub fn on(&self) {
        let chunk = Chunk::from_file("e.wav");
        match chunk {
            Ok(chunk) => {
                Channel::all().play(&chunk, 0).expect("Failed to play music");
            },
            Err(e) => {
                println!("Error loading file: {}", e);
            }
        }
    }

    pub fn off(&self) {
        Channel::all().halt();
    }
}