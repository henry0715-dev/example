/*
   https://leetcode.com/problems/richest-customer-wealth/description/
*/

pub fn test() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];

    match maximum_wealth(&accounts) {
        Some(wealth) => println!("Maximum wealth: {wealth}"),
        None => println!("maximum_wealth -  An error occurred while calculating the maximum value from accounts."),
    }
}

fn maximum_wealth(accounts: &[Vec<i32>]) -> Option<i32> {
    accounts.iter().map(|account| account.iter().sum()).max()
}

#[test]
fn tc() {
    let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let check = 6;

    assert_eq!(maximum_wealth(&accounts), Some(check));
}
