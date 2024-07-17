/*
    https://leetcode.com/problems/running-sum-of-1d-array
 */

pub fn test() {
    let nums = vec![1,2,3,4];
    println!("{:?}", running_sum(nums));
}

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());
    ans.push(nums[0]);

    for i in 1..nums.len() {
        let value = ans[i - 1] + nums[i];
        ans.push(value);
    }

    ans
}