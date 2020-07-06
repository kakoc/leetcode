// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

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

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let None = root {
        return 0;
    }

    let root = root.unwrap();
    let (l_max_i, l_max_l) = {
        if let Some(ref l) = root.borrow().left {
            get_len(Rc::clone(&l))
        } else {
            (0, -1)
        }
    };
    let (r_max_i, r_max_l) = {
        if let Some(ref l) = root.borrow().right {
            get_len(Rc::clone(&l))
        } else {
            (0, -1)
        }
    };

    // println!("{} {} \n {} {}", l_max_i, l_max_l, r_max_i, r_max_l);

    std::cmp::max(l_max_l + r_max_l + 2, std::cmp::max(l_max_i, r_max_i))
}

pub fn get_len(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
    let (l_max_i, l_max_l) = {
        if let Some(ref l) = root.borrow().left {
            get_len(Rc::clone(&l))
        } else {
            (0, -1)
        }
    };
    let (r_max_i, r_max_l) = {
        if let Some(ref l) = root.borrow().right {
            get_len(Rc::clone(&l))
        } else {
            (0, -1)
        }
    };

    // println!("{} {} \n {} {}", l_max_i, l_max_l, r_max_i, r_max_l);
    // println!("{}", l_max_l + r_max_l + 2);
    let int = std::cmp::max(std::cmp::max(l_max_i, r_max_i), l_max_l + r_max_l + 2);
    let len = std::cmp::max(l_max_l + 1, r_max_l + 1);

    (int, len)
}

#[test]
fn test_diameter() {
    //     1
    //    / \
    //   2   3
    //  / \
    // 4   5

    let mut tree = TreeNode::new(1);

    let mut l = TreeNode::new(2);
    l.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    l.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let r = TreeNode::new(3);

    tree.left = Some(Rc::new(RefCell::new(l)));
    tree.right = Some(Rc::new(RefCell::new(r)));

    let d = diameter_of_binary_tree(Some(Rc::new(RefCell::new(tree))));

    assert_eq!(d, 3);
}
