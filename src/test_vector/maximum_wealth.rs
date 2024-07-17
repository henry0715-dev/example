/*
    https://leetcode.com/problems/richest-customer-wealth/description/
 */
pub fn test() {
    let accounts: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![3, 2, 1],
    ];

    println!("{}", maximum_wealth(accounts));

}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for account in accounts.iter().enumerate() {
        let sum: i32 = account.1.into_iter().sum();
        if ans <= sum {
            ans = sum;
        }
    }

    ans
}