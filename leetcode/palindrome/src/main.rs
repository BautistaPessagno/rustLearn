fn main() {
    let x = [121, -121, 10, 11];
    for i in x {
        println!("{}: {}", i, solution(i));
    }
}

fn solution(x: i32) -> bool {
    if x < 10 {
        return false;
    }
    let mut current = x;
    //paso a vec
    let mut v: Vec<i32> = Vec::new();
    while current >= 10 {
        let left = current % 10;
        current = current / 10;
        v.push(left);
    }
    v.push(current);
    let stop = v.len() / 2;
    for i in 0..=stop {
        if v[i] != v[v.len() - 1 - i] {
            return false;
        }
    }
    true
}
