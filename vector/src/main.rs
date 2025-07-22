fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //no se puede hacer lo siguiente porque cuenta como un mutable borrow
    //v.push(6);

    println!("the first element is: {first}");
}
