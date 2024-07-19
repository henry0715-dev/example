/*
   https://leetcode.com/problems/concatenation-of-array
*/
pub fn test() {
    let nums = [1, 2, 1];
    let ans = get_concatenation(&nums);
    println!("{ans:?}");
}

fn get_concatenation(nums: &[i32]) -> Vec<i32> {
    let len = nums.len();
    let mut ans = Vec::with_capacity(2 * len);

    for &num in nums.iter().cycle().take(2 * len) {
        ans.push(num);
    }

    ans
}

#[test]
fn tc() {
    let nums = [1, 2, 1];
    let result = get_concatenation(&nums);
    let check = vec![1, 2, 1, 1, 2, 1];
    assert_eq!(result, check);
}
