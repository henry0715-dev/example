/*
   https://leetcode.com/problems/decompress-run-length-encoded-list/
   1. 입력 받은 nums 배열의 pair는 "0,1", "2,3"과 같이 2*i, 2*i+1의 index의 값을 갖음
   2. 2*i+1의 index의 값이 2*i index의 값 만큼 output 배열에 추가 해야함
*/
pub fn test() {
    let nums = vec![1, 2, 3, 4];
    println!(
        "decompress_rl_elist result : {:?}",
        decompress_rl_elist(&nums)
    );
    println!(
        "decompress_rl_elis_perf result : {:?}",
        decompress_rl_elis_perf(&nums)
    );
}

pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
    let mut ans = Vec::new();

    for i in 0..nums.iter().len() / 2 {
        let freq = nums[2 * i];
        let value = nums[2 * i + 1];

        for _j in 0..freq {
            ans.push(value);
        }
    }

    ans
}

// 가독성 개선 코드
pub fn decompress_rl_elis_perf(nums: &[i32]) -> Vec<i32> {
    let mut ans = Vec::new();

    let iter = nums.chunks_exact(2);

    for chunk in iter {
        ans.extend(std::iter::repeat(chunk[1]));
    }

    ans
}

#[test]
fn tc() {
    let nums = vec![1, 2, 3, 4];
    let result = decompress_rl_elist(&nums);
    let check = vec![2, 4, 4, 4];
    assert_eq!(result, check);
}
