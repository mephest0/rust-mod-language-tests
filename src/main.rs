mod lua;
mod python;
mod timer;

fn main() {
    let number_of_runs = 500000;

    lua::lua(number_of_runs);
    python::python(number_of_runs);
}
