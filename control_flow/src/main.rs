fn main() {
    let f = f_c(27);

    println!("Farenheit: {f}");

    let c = c_f(f);

    println!("Celsius: {c}");

    let mut x = 0;

    while x <= 10 {
        let f = fibo(x);
        println!("{f}");
        x = x + 1;
    }
}

fn c_f(x: i32) -> i32 {
    x - 273
}

fn f_c(x: i32) -> i32 {
    x + 273
}

fn fibo(x: i32) -> i32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibo(x - 1) + fibo(x - 2)
    }
}
