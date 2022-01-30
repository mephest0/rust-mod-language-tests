extern crate hlua;
use hlua::Lua;
use crate::lua::run_lua;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

mod lua;

fn main() {
    let mut lua = Lua::new();

    lua::lua_init(&mut lua);

    for _ in 0..10 {
        let x = run_lua(&mut lua, 10);

        println!("(lua says) x: {}", x);
    }

    python(10);
}

fn python(i: i32) {
    let _x: PyResult<()> = Python::with_gil(|py| {
        let fun = PyModule::from_code(
            py,
            r#"def fact(n):
    print(str(n))
    return 1 if (n==1 or n==0) else n * fact(n - 1);"#,
            "factorial.py",
            "factorial",
        )?.getattr("fact")?;

        // start time
        for _ in 0..i {
            let _x: i32 = fun.call1((10,))?.extract()?;
        }
        // stop time

        Ok(())
    });
}
