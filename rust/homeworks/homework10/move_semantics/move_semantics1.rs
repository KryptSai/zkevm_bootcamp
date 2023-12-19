// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vecs = vec;

    vecs.push(22);
    vecs.push(44);
    vecs.push(66);

    vecs
}
