mod lua;
mod python;
mod timer;
mod loader;

fn main() {
    let number_of_runs_factorial = 5_000_000;

    lua::factorial_iterative(number_of_runs_factorial);
    python::factorial_iterative(number_of_runs_factorial);
    lua::factorial_recursive(number_of_runs_factorial);
    python::factorial_recursive(number_of_runs_factorial);
}
