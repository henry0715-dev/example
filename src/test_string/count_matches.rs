/*
    https://leetcode.com/problems/count-items-matching-a-rule/
    1. rule_key에 맞는 items의 column 구하기
    2. rule_value에 맞는 items 카운팅 구하기
 */
pub fn test() {
    let items: Vec<Vec<String>> = vec![
        vec![String::from("phone"), String::from("blue"), String::from("pixel")],
        vec![String::from("computer"), String::from("silver"), String::from("lenovo")],
        vec![String::from("phone"), String::from("gold"), String::from("iphone")],
    ];
    let rule_key = String::from("color");
    let rule_value = String::from("silver");

    println!("count_matches result : {}", count_matches(items, rule_key, rule_value));
}

fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let search_idx = get_column_by_rule_key(rule_key);

    items.iter()
        .filter(|item| item[search_idx] == rule_value)
        .count() as i32
}

fn get_column_by_rule_key(rule_key: String) -> usize {
    match rule_key.as_str() {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => panic!("rule_key."),
    }
}