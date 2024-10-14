use std::sync::Arc;

use mlua::Lua;

use super::{Synth, SynthRef};

#[derive(Clone, Debug)]
pub struct Shift {
    source: SynthRef,
    delay: f32,
}

impl Shift {
    pub fn constructor(lua: &Lua) -> mlua::Function<'_> {
        lua.create_function(|_, (source, delay): (SynthRef, f32)| {
            Ok(SynthRef(Arc::new(Synth::Shift(Shift::new_desc(
                source, delay,
            )))))
        })
        .unwrap()
    }

    pub fn new_desc(source: SynthRef, delay: f32) -> Shift {
        Shift { source, delay }
    }

    pub fn sample(&self, phase: f32) -> f32 {
        if phase < self.delay {
            0.0
        } else {
            self.source.sample(phase - self.delay)
        }
    }
}
