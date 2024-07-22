/*
   https://leetcode.com/problems/richest-customer-wealth/description/
*/

pub fn test() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];

    println!("maximum_wealth result is : {:?}", maximum_wealth(&accounts));
}

fn maximum_wealth(accounts: &[Vec<i32>]) -> Result<i32, &str> {
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .ok_or("An error occurred while calculating the maximum value from accounts.")
}

#[test]
fn tc() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let empty_accounts: Vec<Vec<i32>> = Vec::new();
    let check = 6;

    assert_eq!(maximum_wealth(&accounts), Ok(check));
    assert_eq!(
        maximum_wealth(&empty_accounts),
        Err("An error occurred while calculating the maximum value from accounts.")
    );
}
