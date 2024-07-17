/*
    https://leetcode.com/problems/concatenation-of-array
 */
pub fn test() {
    let nums = vec![1, 2, 1];
    let ans = get_concatenation(&nums);
    println!("{ans:?}");
}

fn get_concatenation(nums: &Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ans = Vec::with_capacity(2 * len);

    ans.extend_from_slice(nums);
    ans.extend_from_slice(nums);
    ans
}