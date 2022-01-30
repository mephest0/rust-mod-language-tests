extern crate hlua;
use hlua::Lua;
use crate::lua::run_lua;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

mod lua;
mod python;

fn main() {
    let mut lua = Lua::new();

    lua::lua_init(&mut lua);

    for _ in 0..10 {
        let x = run_lua(&mut lua, 10);

        println!("(lua says) x: {}", x);
    }

    python::python(10);
}
