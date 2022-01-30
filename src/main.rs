mod lua;
mod python;

fn main() {
    let number_of_runs = 500000;

    println!("start lua");
    lua::lua(number_of_runs);
    println!("lua complete, start Python");
    python::python(number_of_runs);
    println!("Python complete");
}
