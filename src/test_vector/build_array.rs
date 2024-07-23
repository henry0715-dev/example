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
            let Ok(num_idx) = usize::try_from(num) else {
                return Err("Invalid index error: The given number cannot be converted to usize.");
            };

            nums.get(num_idx)
                .copied()
                .ok_or("Index error: Index out of bounds error.")
        })
        .collect()
}

#[test]
fn tc() {
    let nums = vec![0, 2, 1, 5, 3, 4];
    let result = build_array(&nums).ok().unwrap();
    let check = vec![0, 1, 2, 4, 5, 3];

    assert_eq!(result, check);

    let nums = vec![0, 2, -1, 5, 3, 4];
    let result = build_array(&nums);
    assert_eq!(
        result.unwrap_err(),
        "Invalid index error: The given number cannot be converted to usize."
    );

    let nums = vec![0, 2, 1, 6, 3, 4];
    let result = build_array(&nums);
    assert_eq!(
        result.unwrap_err(),
        "Index error: Index out of bounds error."
    );
}
