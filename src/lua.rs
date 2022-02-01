extern crate hlua;
use hlua::Lua;
use crate::loader::load_file;
use crate::timer;

pub fn factorial_iterative(i: i32) {
    let mut lua = Lua::new();

    lua_init(&mut lua, "scripts/factorial.lua");

    let mut x: i32 = -1;
    timer::start();
    for _ in 0..i {
        x = run_iterative(&mut lua, 12);
    }

    timer::stop("Lua\t\tfactorial\titerative");
    println!("(lua) last result {} ", x);
}

pub fn factorial_recursive(i: i32) {
    let mut lua = Lua::new();

    lua_init(&mut lua, "scripts/factorial_recursive.lua");

    let mut x: i32 = -1;
    timer::start();
    for _ in 0..i {
        x = run_recursive(&mut lua, 12);
    }

    timer::stop("Lua\t\tfactorial\trecursive");
    println!("(lua) last result {} ", x);
}

fn lua_init(lua: &mut Lua, file: &str) {
    let code = load_file(&file);

    // load function
    lua.execute::<()>(&code).unwrap();
}

fn run_iterative(lua: &mut Lua, n: i32) -> i32 {
    lua.set("x", n);

    lua.execute::<()>("x = factorial(x)").unwrap();

    // get result
    let x: i32 = lua.get("x").unwrap();

    x
}

fn run_recursive(lua: &mut Lua, n: i32) -> i32 {
    lua.set("x", n);

    lua.execute::<()>("x = factorial(x)").unwrap();

    lua.get("x").unwrap()
}
