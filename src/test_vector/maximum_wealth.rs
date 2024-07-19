/*
   https://leetcode.com/problems/richest-customer-wealth/description/
*/
pub fn test() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];

    println!("{}", maximum_wealth(&accounts));
}

fn maximum_wealth(accounts: &[Vec<i32>]) -> i32 {
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .unwrap_or(0)
}

#[test]
fn tc() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let result = maximum_wealth(&accounts);
    let check = 6;
    assert_eq!(result, check);
}
