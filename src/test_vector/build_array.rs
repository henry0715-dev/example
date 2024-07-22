/*
    https://leetcode.com/problems/build-array-from-permutation
*/

pub fn test() {
    let nums = vec![0, 2, 1, 5, 3, 4];
    println!("{:?}", build_array(&nums));
}

fn build_array(nums: &[i32]) -> Result<Vec<i32>, &str> {
    nums.iter()
        .map(|&num| {
            let num_idx = usize::try_from(num).expect("InvalidIndex num_idx");
            nums.get(num_idx).copied().ok_or("Index out of bounds")
        })
        .collect()
}

#[test]
fn tc() {
    let nums = vec![0, 2, 1, 5, 3, 4];
    let result = build_array(&nums).ok().unwrap();
    let check = vec![0, 1, 2, 4, 5, 3];

    assert_eq!(result, check);
}
