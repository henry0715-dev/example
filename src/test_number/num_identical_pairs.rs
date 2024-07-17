/*
    https://leetcode.com/problems/number-of-good-pairs/
    1. for문을 순회하며 key 값에 일치하는 값이 있을 경우 value 값을 증가
    2. 현재 배열의 값과 hashMap의 키 값과 일치 하는 경우 ans 값을 증가
 */
use std::collections::{HashMap};

pub fn test() {
    let nums = vec![1,2,3,1,1,3];
    println!("{}", num_identical_pairs(&nums));

    // let nums = vec![1,1,1,1];
    // println!("{}", num_identical_pairs(&nums));
    //
    // let nums = vec![1,2,3];
    // println!("{}", num_identical_pairs(&nums));
}

fn num_identical_pairs(nums: &Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut nums_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let opt_value = nums_map.get(&nums[i]);

        if opt_value.is_some() {
            ans += opt_value.unwrap();
        }

        let counter = nums_map.entry(nums[i]).or_insert(0);
        *counter += 1;
    }

    ans
}