fn main() {
    let nums = vec![5, 4, 2, 3];
    println!("input: {nums:?}");
    let ans = solution(nums);
    println!("outout: {ans:?}");
}

fn solution(nums: Vec<i32>) -> Vec<i32> {
    let mut arr = nums.clone();
    let mut ans: Vec<i32> = Vec::new();
    while !arr.is_empty() {
        let alice = getmin(&mut arr);
        let bob = getmin(&mut arr);
        ans.push(bob);
        ans.push(alice);
    }
    ans
}

fn getmin(nums: &mut Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut index = 0;
    for (i, v) in nums.iter().enumerate() {
        if *v < min {
            min = *v;
            index = i;
        }
    }
    nums.swap_remove(index)
}
