/*
    https://leetcode.com/problems/defanging-an-ip-address/
    문자 변환 "." with "[.]"
 */
pub fn test() {
    let address = "1.1.1.1".to_string();
    println!("defang_i_paddr result : {}", defang_i_paddr(address));
}

fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}