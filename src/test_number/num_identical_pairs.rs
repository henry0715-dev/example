/*
   https://leetcode.com/problems/number-of-good-pairs/
   1. for문을 순회하며 key 값에 일치하는 값이 있을 경우 value 값을 증가
   2. 현재 배열의 값과 hashMap의 키 값과 일치 하는 경우 ans 값을 증가
*/
use std::collections::HashMap;

pub fn test() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    println!(
        "num_identical_pairs result : {}",
        num_identical_pairs(&nums)
    );

    let nums = vec![1, 1, 1, 1];
    println!(
        "num_identical_pairs result : {}",
        num_identical_pairs(&nums)
    );

    let nums = vec![1, 2, 3];
    println!(
        "num_identical_pairs result : {}",
        num_identical_pairs(&nums)
    );
}

fn num_identical_pairs(nums: &[i32]) -> i32 {
    let ans_tuple = nums
        .iter()
        .fold((0, HashMap::new()), |(mut ans, mut nums_map), num| {
            ans += nums_map.get(&num).unwrap_or(&0);
            *nums_map.entry(num).or_insert(0) += 1;
            (ans, nums_map)
        });
    ans_tuple.0
}

#[test]
fn tc() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    let result = num_identical_pairs(&nums);
    let check = 4;
    assert_eq!(result, check);
}
