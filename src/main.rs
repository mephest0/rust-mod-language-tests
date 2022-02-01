mod lua;
mod python;
mod timer;
mod loader;
mod utils;

fn main() {
    let number_of_runs_factorial = 5_000_000;

    lua::factorial_iterative(number_of_runs_factorial);
    python::factorial_iterative(number_of_runs_factorial);
    lua::factorial_recursive(number_of_runs_factorial);
    python::factorial_recursive(number_of_runs_factorial);

    // Data for loading test
    let vector1k = utils::fill_vector(1_000);
    let vector1m = utils::fill_vector(1_000_000);
    let vector1b = utils::fill_vector(500_000_000);

    python::load_data(&vector1b, 0);
    python::load_data(&vector1b, 500_000_000);
    python::load_data(&vector1m, 0);
    python::load_data(&vector1m, 1_000_000);
    python::load_data(&vector1k, 0);
    python::load_data(&vector1k, 1_000);
}
