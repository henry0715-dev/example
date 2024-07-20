/*
   https://leetcode.com/problems/decode-the-message/
   1. key값으로 받은 String whitespace제거
   2. key value 매핑 만들기 (key값 내부 중복제거되서 들어가도록)
   3. message로 받은 값을 매핑의 value로 치환해서 결과 값 넘겨주기
   4. message로 받은 값 중 whitespace의 경우에는 동일하게 whitespace로 만들어 주기
*/
use std::collections::HashMap;

static BASE_TABLE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn test() {
    let key = "the quick brown fox jumps over the lazy dog";
    let message = "vkbs bs t suepuv";
    println!("{}", decode_message(key, message));
}

fn decode_message(key: &str, message: &str) -> String {
    let mut base_table_idx: usize = 0;
    let mut key_map = HashMap::new();

    key.chars().filter(|&c| c != ' ').for_each(|c| {
        key_map.entry(c).or_insert_with(|| {
            let val = BASE_TABLE[base_table_idx];
            base_table_idx += 1;
            val
        });
    });

    message
        .chars()
        .map(|c| *key_map.get(&c).unwrap_or(&c))
        .collect()
}

#[test]
fn tc() {
    let key = "the quick brown fox jumps over the lazy dog";
    let message = "vkbs bs t suepuv";
    let result = decode_message(key, message);
    let check = "this is a secret".to_string();
    assert_eq!(result, check);
}
