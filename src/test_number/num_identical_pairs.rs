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

    // let nums = vec![1,1,1,1];
    // println!("{}", num_identical_pairs(&nums));
    //
    // let nums = vec![1,2,3];
    // println!("{}", num_identical_pairs(&nums));
}

fn num_identical_pairs(nums: &[i32]) -> i32 {
    let mut ans = 0;
    let mut nums_map: HashMap<i32, i32> = HashMap::new();

    for item in nums {
        let opt_value = nums_map.get(item);

        if let Some(&value) = opt_value {
            ans += value;
        }

        *nums_map.entry(*item).or_insert(0) += 1;
    }

    ans
}

#[test]
fn tc() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    let result = num_identical_pairs(&nums);
    let check = 4;
    assert_eq!(result, check);
}
