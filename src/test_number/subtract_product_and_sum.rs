/*
   https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
   1. 숫자를 배열로 변환 작업
   2. 배열 곱셈 구하기
   3. 배열 합 구하기
*/
pub fn test() {
    println!(
        "subtract_product_and_sum result : {}",
        subtract_product_and_sum(234)
    );
    println!(
        "subtract_product_and_sum result : {}",
        subtract_product_and_sum(4421)
    );
}

fn subtract_product_and_sum(n: i32) -> i32 {
    let n_array: Vec<i32> = get_array_by_num(n);
    let product: i32 = n_array.iter().product();
    let sum: i32 = n_array.iter().sum();

    product - sum
}

fn get_array_by_num(num: i32) -> Vec<i32> {
    num.to_string()
        .chars()
        .filter_map(|c| Option::from(i32::try_from(c.to_digit(10).unwrap()).unwrap()))
        .collect()
}

#[test]
fn tc() {
    let result = subtract_product_and_sum(234);
    let check = 15;
    assert_eq!(result, check);
}
