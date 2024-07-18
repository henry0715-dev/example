/*
   https://leetcode.com/problems/shuffle-the-array/
*/

pub fn test() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    println!("{:?}", shuffle(nums, n))
}

fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let len = n as usize;
    let mut ans: Vec<i32> = Vec::with_capacity(nums.len());

    for i in 0..len {
        ans.push(nums[i]);
        ans.push(nums[i + len]);
    }

    ans
}

#[test]
fn tc() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    let result = shuffle(nums, n);
    let check = vec![2, 3, 5, 4, 1, 7];
    assert_eq!(result, check);
}
