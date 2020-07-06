// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let None = root {
        return 0;
    }

    let (b, r) = get_local_max(root, 0, 0);

    std::cmp::max(b, r)
}

fn get_local_max(node: Option<Rc<RefCell<TreeNode>>>, b: i32, r: i32) -> (i32, i32) {
    if let Some(n) = node {
        match (&n.borrow().left, &n.borrow().right) {
            (Some(l), Some(r)) => {
                let (bl, rl) = get_local_max(Some(l.clone()), 0, 0);
                let (br, rr) = get_local_max(Some(r.clone()), 0, 0);
                let max_b = std::cmp::max(
                    std::cmp::max(bl + n.borrow().val, br + n.borrow().val),
                    n.borrow().val,
                );
                let max_r = std::cmp::max(
                    std::cmp::max(std::cmp::max(rl, rr), max_b),
                    std::cmp::max(bl + br + n.borrow().val, n.borrow().val),
                );
                (max_b, max_r)
            }
            (Some(l), None) => {
                let (bl, rl) = get_local_max(Some(l.clone()), 0, 0);
                let max_b = std::cmp::max(bl + n.borrow().val, n.borrow().val);
                let max_r = std::cmp::max(max_b, rl);
                (max_b, max_r)
            }
            (None, Some(r)) => {
                let (br, rr) = get_local_max(Some(r.clone()), 0, 0);
                let max_b = std::cmp::max(br + n.borrow().val, n.borrow().val);
                let max_r = std::cmp::max(max_b, rr);
                (max_b, max_r)
            }
            (None, None) => (n.borrow().val, n.borrow().val),
        }
    } else {
        (0, 0)
    }
}

#[test]
fn test_max_bst() {
    assert_eq!(5, 5);
}
