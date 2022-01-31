mod lua;
mod python;
mod timer;
mod loader;

fn main() {
    let number_of_runs = 5_000_000;

    lua::lua_factorial_iterative(number_of_runs);
    python::python_factorial_iterative(number_of_runs);
}
