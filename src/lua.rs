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
