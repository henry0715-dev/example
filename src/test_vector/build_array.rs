/*
    https://leetcode.com/problems/build-array-from-permutation
*/

pub fn test() {
    let nums = vec![0,2,1,5,3,4];
    println!("{:?}", build_array(&nums));
}


fn build_array(nums: &Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());

    for &num in nums {
        let idx = num as usize;
        let value = nums[idx];
        ans.push(value);
    }

    ans
}