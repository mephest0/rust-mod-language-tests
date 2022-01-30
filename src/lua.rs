use hlua::Lua;

pub fn lua_init(mut lua: Lua) -> Lua {
    // load function
    lua.execute::<()>("function factorial(n)
    local x = 1
    for i = 2, n do
        x = x * i
    end
    return x
end").unwrap();

    lua
}

pub fn run_lua(mut lua: Lua, n: i32) -> i32 {
    lua.set("x", n);

    lua.execute::<()>("x = factorial(x)").unwrap();

    // get result
    let x: i32 = lua.get("x").unwrap();

    x
}
