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

pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() == 0 {
        return None;
    }

    let mut i = 0;
    let root = construct(
        &preorder,
        &mut i,
        Some(&preorder[0]),
        std::i32::MIN,
        std::i32::MAX,
    )
    .unwrap();

    Some(Rc::new(RefCell::new(root)))
}

pub fn construct(
    vec: &Vec<i32>,
    i: &mut usize,
    key: Option<&i32>,
    min: i32,
    max: i32,
) -> Option<TreeNode> {
    if !(*i < vec.len()) {
        return None;
    }

    if let None = key {
        return None;
    }

    let key = *key.unwrap();

    if key > min && key < max {
        let mut root = TreeNode::new(key);
        *i += 1;

        if *i < vec.len() {
            if let Some(n) = construct(vec, i, vec.get(*i), min, key) {
                root.left = Some(Rc::new(RefCell::new(n)));
            } else {
                root.left = None;
            }

            if let Some(n) = construct(vec, i, vec.get(*i), key, max) {
                root.right = Some(Rc::new(RefCell::new(n)));
            } else {
                root.right = None;
            }
        }

        return Some(root);
    }

    return None;
}

// fn change(n: &mut usize) {
//     *n += 1;
// }

#[test]
fn test_bst_from_preorder() {
    let mut i = vec![4, 2];
    let a = bst_from_preorder(i);

    println!("{:?}", a);
}
