extern crate hlua;
use hlua::Lua;
use crate::lua::run_lua;

mod lua;

fn main() {
    println!("Hello, world!");

    let mut lua = Lua::new();

    lua = lua::lua_init(lua);

    let x = run_lua(lua, 10);

    println!("(lua says) x: {}", x);
}
