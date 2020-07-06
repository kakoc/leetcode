// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut h = &head.unwrap();
    let mut v = vec![h];
    while let Some(n) = &h.next {
        h = &n;
        v.push(h);
    }

    let odd = v.len() % 2 == 0;
    if odd {
        Some(v[v.len() / 2 + 1].clone())
    } else {
        Some(v[(v.len() / 2) as usize].clone())
    }
}

#[test]
fn test_ll() {}
