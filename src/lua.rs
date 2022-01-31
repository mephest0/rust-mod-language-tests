extern crate hlua;
use hlua::Lua;
use crate::loader::load_file;
use crate::timer;

pub fn lua(i: i32) {
    let mut lua = Lua::new();

    lua_init(&mut lua);

    let mut x: i32 = -1;
    timer::start();
    for _ in 0..i {
        x = run_lua(&mut lua, 12);
    }
    timer::stop("Lua, factorial, iterative");

    println!("(lua) last result {}", x);
}

fn lua_init(lua: &mut Lua) {
    let code = load_file("scripts/factorial.lua");

    // load function
    lua.execute::<()>(&code).unwrap();
}

fn run_lua(lua: &mut Lua, n: i32) -> i32 {
    lua.set("x", n);

    lua.execute::<()>("x = factorial(x)").unwrap();

    // get result
    let x: i32 = lua.get("x").unwrap();

    x
}
