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
// impl Solution {
pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
    check(root, &arr[..])
}

// }

fn check(root: Option<Rc<RefCell<TreeNode>>>, arr: &[i32]) -> bool {
    match root {
        None => false,
        (Some(rr)) => {
            if let Some(v) = arr.get(0) {
                if rr.borrow().val != *v {
                    false
                } else {
                    match (&rr.borrow().left, &rr.borrow().right) {
                        (Some(l), Some(r)) => {
                            let lb = check(Some(l.clone()), &arr[1..]);
                            let rb = check(Some(r.clone()), &arr[1..]);

                            if let Some(v) = arr.get(0) {
                                (lb && rr.borrow().val == *v) || (rb && rr.borrow().val == *v)
                            } else {
                                false
                            }
                        }
                        (Some(l), None) => {
                            let lb = check(Some(l.clone()), &arr[1..]);

                            if let Some(v) = arr.get(0) {
                                lb && rr.borrow().val == *v
                            } else {
                                false
                            }
                        }
                        (None, Some(r)) => {
                            let rb = check(Some(r.clone()), &arr[1..]);

                            if let Some(v) = arr.get(0) {
                                // println!("{} {}", rb, r.borrow().val);
                                // println!("here {} {:?} {}", v, arr, rb && r.borrow().val == *v);
                                rb && rr.borrow().val == *v
                            } else {
                                false
                            }
                        }
                        (None, None) => {
                            if let Some(v) = arr.get(0) {
                                rr.borrow().val == *v && arr.len() == 1
                            } else {
                                false
                            }
                        }
                    }
                }
            } else {
                false
            }
        }
    }
}

#[test]
fn test_str_in_bst() {
    let i = vec![1, 2];
    assert_eq!(2, i[1..][0]);

    let i = vec![1];
    assert_eq!(1, i[0..][0]);

    let r = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    let a = is_valid_sequence(r, vec![0, 1, 0, 1]);
    assert_eq!(a, true);
}
