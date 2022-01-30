use pyo3::{PyResult, Python};
use pyo3::prelude::PyModule;
use crate::timer;

pub fn python(i: i32) {
    let _x: PyResult<()> = Python::with_gil(|py| {
        let fun = PyModule::from_code(
            py,
            r#"def fact(n):
    ret = 1
    for i in range(2, n + 1):
        ret = ret * i

    return ret"#,
            "factorial.py",
            "factorial",
        )?.getattr("fact")?;

        let mut x: i32 = -1;

        timer::start();
        for _ in 0..i {
            x = fun.call1((12,))?.extract()?;
        }
        timer::stop("Python, factorial, iterative");

        println!("(py) last result {}", x);
        // stop time

        Ok(())
    });
}
