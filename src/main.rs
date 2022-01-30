mod lua;
mod python;
mod timer;

fn main() {
    let number_of_runs = 5_000_000;

    lua::lua(number_of_runs);
    python::python(number_of_runs);
}
