fn main() {
    let i1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    printMat(&i1);
    println!("");
    printMat(&solution(&i1));
}

fn solution(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let x = matrix.len() - 1;
    let y = matrix[0].len() - 1;

    let mut ans: Vec<Vec<i32>> = vec![vec![0; x + 1]; y + 1];

    for i in 0..=x {
        for w in 0..=y {
            ans[w][i] = matrix[i][w];
        }
    }
    ans
}

fn printMat(mat: &Vec<Vec<i32>>) {
    for x in mat {
        for y in x {
            print!("{} ", y);
        }
        println!("");
    }
}
