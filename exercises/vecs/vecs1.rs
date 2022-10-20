// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

use std::ops::Add;

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain 
    let mut v = Vec::from(a);// TODO: declare your vector here with the macro for vectors
    let i = &v[0];

    let y = v.get(2);
    match y {
        Some(t) => println!("{}", t),
        None => println!("None")
    }
    for i in &mut v {
        *i += 2;
        *i -= 2;
    }

    (a, v)
}

fn test(s: &mut String) {
    *s = String::from("asdasdasd");
}

fn add(i: &mut i32) {
    i.add(2);
    // *i += 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
