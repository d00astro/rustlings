// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec0 = Vec::new();

    vec0.push(11);

    let mut vec1 = fill_vec(&vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for v in vec.iter() {
        new_vec.push(*v);
    }

    new_vec.push(22);
    new_vec.push(44);
    new_vec.push(66);

    new_vec
}
