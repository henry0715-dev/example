/*
   https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
*/

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn test() {
    let mut node1 = ListNode::new(1);
    let node2 = ListNode::new(0);
    let node3 = ListNode::new(1);

    node1.next = Some(Box::new(node2));

    if let Some(ref mut node1_next) = node1.next {
        node1_next.next = Some(Box::new(node3));
    }

    let head = Some(Box::new(node1));
    println!("{}", get_decimal_value(head));
}

fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut num = 0;
    let mut current_node = head.as_ref();

    while let Some(node) = current_node {
        num = num * 2 + node.val;
        current_node = node.next.as_ref();
    }

    num
}
