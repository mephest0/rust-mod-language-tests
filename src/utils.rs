pub fn fill_vector(n: i32) -> Vec<i32> {
    let mut vector = Vec::new();

    for i in 0..n + 1 {
        vector.push(i);
    }

    vector
}
