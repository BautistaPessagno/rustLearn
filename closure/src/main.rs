fn main() {
    let add1 = |x| x + 1;

    let n: i32 = add1(1);

    println!("{n}");

    let list = vec![1, 2, 3];

    println!("before defining closure: {list:?}");

    let onlly_borrows = || println!("from closure: {list:?}");

    println!("before calling closure: {list:?}");

    onlly_borrows();
    
    println!("after calling  closure: {list:?}");
    
    let it = list.iter();
    for i in it{
    println!("{i}");
    }
}

