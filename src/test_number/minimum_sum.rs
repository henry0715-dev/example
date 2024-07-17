/*
    https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
    2자리 숫자 최소 값을 더하기
    1. 숫자 문자열 치환
    2. 오름차순 정렬
    3. char_array[0][2] + char_array[1][3] 더하기
 */

pub fn test() {
    println!("{}", minimum_sum(2932));
    println!("{}", minimum_sum(4009));

}

fn minimum_sum(num: i32) -> i32 {
    let char_array = get_ordered_char_array_by_nums(num);
    let num1: i32 = format!("{}{}", char_array[0], char_array[2]).parse().unwrap();
    let num2: i32 = format!("{}{}", char_array[1], char_array[3]).parse().unwrap();

    num1 + num2
}

fn get_ordered_char_array_by_nums(num: i32) -> Vec<char> {
    let mut char_array: Vec<char> = num.to_string().chars().collect();
    char_array.sort_by(|a, b| a.cmp(b));
    char_array
}

