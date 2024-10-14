use std::{f32::consts::PI, fs, path::PathBuf, thread, time::Duration};

use clap::Parser;
use mlua::{AnyUserData, Lua};
use sdl2::audio::{AudioCallback, AudioSpec, AudioSpecDesired};
use synth::{Synth, SynthRef};

mod synth;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    script_path: PathBuf,
}

struct Callback {
    synth: SynthRef,
    spec: AudioSpec,
    phase: f32,
}

impl Callback {
    fn new(synth: SynthRef, spec: AudioSpec) -> Callback {
        Callback {
            synth,
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

impl AudioCallback for Callback {
    type Channel = f32;

    fn callback(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.synth.sample(self.phase);
            self.next_sample();
        }
    }
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.script_path).expect("Cannot read script file");
    let mut lua = Lua::new();
    Synth::install_constructors(&mut lua);
    let result: SynthRef = lua.load(&code).eval::<AnyUserData>().unwrap().take().unwrap();
    println!("{:?}", result);

    let sdl = sdl2::init().unwrap();
    let sdl_audio = sdl.audio().unwrap();
    let spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };
    let device = sdl_audio
        .open_playback(None, &spec, |spec| Callback::new( result, spec))
        .unwrap();
    device.resume();

    thread::sleep(Duration::from_secs(10));
}
