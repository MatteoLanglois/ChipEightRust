use sdl2::audio::{AudioCallback, AudioDevice};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                (self.volume * 255.0) as u8
            } else {
                0
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Speaker {
    device: AudioDevice<SquareWave>,
}

impl Speaker {
    pub(crate) fn new(audio_subsystem: &sdl2::AudioSubsystem) -> Speaker {
        let device = audio_subsystem.open_playback(None, &sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        }, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }).unwrap();

        Speaker {
            device
        }
    }

    pub fn on(&self) {
        self.device.resume();
    }

    pub fn off(&self) {
        self.device.pause();
    }
}