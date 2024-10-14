use std::{ops::Deref, sync::Arc};

use arithmetic::{Arithmetic, ArithmeticOp};
use mlua::{FromLua, Lua, UserData};
use wave::Wave;

mod arithmetic;
mod wave;

#[derive(Clone, Debug)]
pub enum Synth {
    Constant(f32),
    Wave(Wave),
    Arithmetic(Arithmetic),
}

#[derive(Clone, Debug)]
pub struct SynthRef(Arc<Synth>);

impl Deref for SynthRef {
    type Target = Synth;

    fn deref(&self) -> &Synth {
        &self.0
    }
}

impl UserData for SynthRef {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method("__add", |_, this, other: SynthRef| {
            Ok(SynthRef(Arc::new(Synth::Arithmetic(Arithmetic::new_desc(
                ArithmeticOp::Add,
                vec![this.clone(), other],
            )))))
        });
        methods.add_meta_method("__mul", |_, this, other: SynthRef| {
            Ok(SynthRef(Arc::new(Synth::Arithmetic(Arithmetic::new_desc(
                ArithmeticOp::Mul,
                vec![this.clone(), other],
            )))))
        });
    }
}

impl<'lua> FromLua<'lua> for SynthRef {
    fn from_lua(value: mlua::Value<'lua>, _lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        if let Some(value) = value.as_f32() {
            Ok(SynthRef(Arc::new(Synth::Constant(value))))
        } else if let Some(value) = value.as_i32() {
            Ok(SynthRef(Arc::new(Synth::Constant(value as f32))))
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
            Self::Constant(value) => *value,
            Self::Wave(wave) => wave.sample(phase),
            Self::Arithmetic(arithmetic) => arithmetic.sample(phase),
        }
    }
}
