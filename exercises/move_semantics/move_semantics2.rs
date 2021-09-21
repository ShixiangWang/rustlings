// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);
    // method 1: use .clone() method
    // method 2: use reference &vec0 here and clone() vec inside function
    // method 3: Make `fill_vec` *mutably* borrow its argument (which will need to be
   // mutable), modify it directly, then not return anything. Then you can get rid
   // of `vec1` entirely -- note that this will change what gets printed by the
   // first `println!`

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
