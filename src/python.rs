use pyo3::{PyResult, Python};
use pyo3::prelude::PyModule;
use crate::loader::load_file;
use crate::timer;

pub fn factorial_iterative(i: i32) {
    let code = load_file("scripts/factorial.py");

    let _x: PyResult<()> = Python::with_gil(|py| {
        let fun = PyModule::from_code(py, &code, "factorial.py", "factorial")?.getattr("fact")?;

        let mut x: i32 = -1;

        timer::start();
        for _ in 0..i {
            x = fun.call1((12,))?.extract()?;
        }

        timer::stop("Python\tfactorial\titerative");
        println!("(py) last result {} ", x);

        Ok(())
    });
}

pub fn factorial_recursive(i: i32) {
    let code = load_file("scripts/factorial_recursive.py");

    let _x: PyResult<()> = Python::with_gil(|py| {
        let fun = PyModule::from_code(py, &code, "factorial.py", "factorial")?.getattr("fact")?;

        let mut x: i32 = -1;

        timer::start();
        for _ in 0..i {
            x = fun.call1((12,))?.extract()?;
        }

        timer::stop("Python\tfactorial\trecursive");
        println!("(py) last result {} ", x);

        Ok(())
    });

}
