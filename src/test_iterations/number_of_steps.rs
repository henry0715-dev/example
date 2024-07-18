/*
    https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
    1. 입력 받은 숫자에 대해서 홀수/짝수 체크
    2. 홀인 경우 입력 받은 값에 대해 -1 실행
    3. 짝인 경우 나누기 실행
    4. 각 실행에 대해서 count를 증가 시켜줌
*/

pub fn test() {
    println!("number_of_steps result is {}", number_of_steps(14));
    println!("number_of_steps result is {}", number_of_steps(8));
    println!("number_of_steps result is {}", number_of_steps(123));
}
fn number_of_steps(num: i32) -> i32 {
    let mut count = 0;
    let mut current = num;

    while current > 0 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current -= 1;
        }
        count += 1;
    }

    count
}

#[test]
fn tc() {
    let result = number_of_steps(14);
    let check = 6;

    assert_eq!(result, check);
}
