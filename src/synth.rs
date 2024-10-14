use std::{ops::Deref, sync::Arc};

use mlua::{FromLua, Lua, UserData};
use wave::Wave;

mod wave;

#[derive(Clone, Debug)]
pub enum Synth {
    Constant { value: f32 },
    Wave(Wave),
}

#[derive(Clone, Debug)]
pub struct SynthRef(Arc<Synth>);

impl Deref for SynthRef {
    type Target = Synth;

    fn deref(&self) -> &Synth {
        &self.0
    }
}

impl UserData for SynthRef {}

impl<'lua> FromLua<'lua> for SynthRef {
    fn from_lua(value: mlua::Value<'lua>, _lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        if let Some(value) = value.as_f32() {
            Ok(SynthRef(Arc::new(Synth::Constant { value })))
        } else if let Some(value) = value.as_i32() {
            Ok(SynthRef(Arc::new(Synth::Constant {
                value: value as f32,
            })))
        } else if let Some(userdata) = value.as_userdata() {
            let synthref: SynthRef = userdata.borrow::<SynthRef>().unwrap().clone();
            Ok(synthref)
        } else {
            panic!("{:?}", value)
        }
    }
}

impl Synth {
    pub fn install_constructors(lua: &mut Lua) {
        lua.globals().set("wave", Wave::constructor(lua)).unwrap();
    }

    pub fn sample(&self, phase: f32) -> f32 {
        match self {
            Self::Constant { value } => *value,
            Self::Wave(wave) => wave.sample(phase),
        }
    }
}
