use std::sync::Arc;

use mlua::{FromLua, Lua};

use super::{Synth, SynthRef};

#[derive(Clone, Debug)]
pub struct SplinePoint(f32, f32);

impl<'lua> FromLua<'lua> for SplinePoint {
    fn from_lua(value: mlua::Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
        let vt = value.as_table().unwrap();
        Ok(SplinePoint(vt.get(1).unwrap(), vt.get(2).unwrap()))
    }
}

#[derive(Clone, Debug)]
pub struct Spline {
    points: Vec<SplinePoint>,
}

impl Spline {
    pub fn constructor(lua: &Lua) -> mlua::Function<'_> {
        lua.create_function(|_, points: Vec<SplinePoint>| {
            Ok(SynthRef(Arc::new(Synth::Spline(Spline::new_desc(points)))))
        })
        .unwrap()
    }

    pub fn new_desc(points: Vec<SplinePoint>) -> Spline {
        Spline { points }
    }

    pub fn sample(&self, phase: f32) -> f32 {
        let mut i = 0;
        let ranges = self.points.len() - 1;
        while i < ranges && (self.points[i].0 >= phase || self.points[i + 1].0 < phase) {
            i += 1
        }
        if i < ranges {
            let SplinePoint(x_start, y_start) = self.points[i];
            let SplinePoint(x_end, y_end) = self.points[i + 1];
            let (x_len, y_len) = (x_end - x_start, y_end - y_start);
            let x_pos = (phase - x_start) / x_len;
            y_start + x_pos * y_len
        } else {
            0.0
        }
    }
}
