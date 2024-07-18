/*
   https://leetcode.com/problems/running-sum-of-1d-array
*/

pub fn test() {
    let nums = vec![1, 2, 3, 4];
    println!("running_sum result : {:?}", running_sum(nums));
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

#[test]
fn tc() {
    let nums = vec![1, 2, 3, 4];
    let result = running_sum(nums);
    let check = vec![1, 3, 6, 10];
    assert_eq!(result, check);
}
