/*
   https://leetcode.com/problems/shuffle-the-array/
*/

pub fn test() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    println!("{:?}", shuffle(&nums, n));
}

fn shuffle(nums: &[i32], n: usize) -> Vec<i32> {
    nums[..n]
        .iter()
        .zip(&nums[n..])
        .flat_map(|(&left, &right)| vec![left, right])
        .collect()
}

#[test]
fn tc() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    let result = shuffle(&nums, n);
    let check = vec![2, 3, 5, 4, 1, 7];
    assert_eq!(result, check);
}
