extern crate hlua;
use hlua::Lua;

mod lua;

fn main() {
    println!("Hello, world!");

    let mut lua = Lua::new();

    lua = lua::lua_init(lua);

    // call function
    lua.set("x", -1);
    lua.execute::<()>("x = factorial(10)").unwrap();

    // get result
    let x: i32 = lua.get("x").unwrap();

    println!("(lua says) x: {}", x);
}
