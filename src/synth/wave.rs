use std::{f32::consts::PI, sync::Arc};

use mlua::Lua;

use super::{Synth, SynthRef};

#[derive(Clone, Debug)]
pub struct Wave {
    frequency: SynthRef,
}

impl Wave {
    pub fn constructor(lua: &Lua) -> mlua::Function<'_> {
        lua.create_function(|_, frequency: SynthRef| {
            Ok(SynthRef(Arc::new(Synth::Wave(Wave::new_desc(frequency)))))
        })
        .unwrap()
    }

    pub fn new_desc(frequency: SynthRef) -> Wave {
        Wave { frequency }
    }

    pub fn sample(&self, phase: f32) -> f32 {
        let multiplier = 2.0 * PI * self.frequency.sample(phase);
        (phase * multiplier).sin()
    }
}
