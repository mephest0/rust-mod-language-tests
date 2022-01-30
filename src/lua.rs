extern crate hlua;
use hlua::Lua;

pub fn lua(i: i32) {
    let mut lua = Lua::new();

    lua_init(&mut lua);

    for _ in 0..i {
        let _x = run_lua(&mut lua, 10);
    }
}

fn lua_init(lua: &mut Lua) {
    // load function
    lua.execute::<()>("function factorial(n)
    local x = 1
    for i = 2, n do
        x = x * i
    end
    return x
end").unwrap();
}

fn run_lua(lua: &mut Lua, n: i32) -> i32 {
    lua.set("x", n);

    lua.execute::<()>("x = factorial(x)").unwrap();

    // get result
    let x: i32 = lua.get("x").unwrap();

    x
}
