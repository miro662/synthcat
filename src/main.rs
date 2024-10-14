use std::{f32::consts::PI, fs, path::PathBuf, thread, time::Duration};

use clap::Parser;
use mlua::Lua;
use sdl2::audio::{AudioCallback, AudioSpec, AudioSpecDesired};

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    script_path: PathBuf,
}

struct Synth {
    freq: f32,
    spec: AudioSpec,
    phase: f32,
}

impl Synth {
    fn new(freq: f32, spec: AudioSpec) -> Synth {
        Synth {
            freq,
            spec,
            phase: 0.0,
        }
    }

    #[inline]
    fn next_sample(&mut self) {
        let sample_len = 1.0 / self.spec.freq as f32;
        self.phase += sample_len;
    }
}

impl AudioCallback for Synth {
    type Channel = f32;

    fn callback(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = (self.freq * 2.0 * PI * self.phase).sin() * 0.2;
            self.next_sample();
        }
    }
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.script_path).expect("Cannot read script file");
    let lua = Lua::new();
    let result = lua.load(&code).eval::<u64>().unwrap();

    let sdl = sdl2::init().unwrap();
    let sdl_audio = sdl.audio().unwrap();
    let spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };
    let device = sdl_audio
        .open_playback(None, &spec, |spec| Synth::new(result as f32, spec))
        .unwrap();
    device.resume();

    thread::sleep(Duration::from_secs(10));
}
