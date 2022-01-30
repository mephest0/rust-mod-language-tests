extern crate hlua;
use hlua::Lua;
use crate::lua::run_lua;

mod lua;

fn main() {
    println!("Hello, world!");

    let mut lua = Lua::new();

    lua::lua_init(&mut lua);

    for _ in 0..10 {
        let x = run_lua(&mut lua, 10);

        println!("(lua says) x: {}", x);
    }
}
