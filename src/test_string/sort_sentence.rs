/*
   https://leetcode.com/problems/sorting-the-sentence/
   1. split 문자열 크기만큼 vec 생성
   2. split 문자열에서 index 번호 파싱
   3. index 번호에 맞는 위치에 문자열 저장
   4. vec join 으로 결과 추출
*/
pub fn test() {
    let s = String::from("is2 sentence4 This1 a3");
    println!("sort_sentence result : {}", sort_sentence(s));
}

fn sort_sentence(s: String) -> String {
    let mut ans: Vec<Option<String>> = vec![None; s.split_whitespace().count()];

    adjust_ans(s, &mut ans);

    ans.into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join(" ")
}

fn adjust_ans(s: String, ans: &mut Vec<Option<String>>) {
    for x in s.split_whitespace() {
        let mut index: usize = 0;
        let mut strs = String::with_capacity(x.len());

        for c in x.chars() {
            if c.is_numeric() {
                index = (c.to_digit(10).unwrap() as usize) - 1;
            } else {
                strs.push(c);
            }
        }
        ans[index] = Some(strs);
    }
}

#[test]
fn tc() {
    let s = String::from("is2 sentence4 This1 a3");

    let result = sort_sentence(s);
    let check = "This is a sentence".to_string();
    assert_eq!(result, check);
}
