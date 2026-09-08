// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

/* 112. Path Sum */
/* 0ms, 2.78MB */

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            // Base case: empty tree has no valid path
            None => false,
            Some(node) => {
                let node_ref = node.borrow();
                let remaining = target_sum - node_ref.val;

                // Leaf node: check whether the remaining sum is exactly zero
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    return remaining == 0;
                }

                // Clone Rc pointers (cheap: only increments reference count)
                // before recursing, since node_ref still borrows 'node' here.
                let left = node_ref.left.clone();
                let right = node_ref.right.clone();
                drop(node_ref); // release the immutable borrow explicitly (optional but clear)

                // Recurse on left and right subtrees
                Self::has_path_sum(left, remaining) || Self::has_path_sum(right, remaining)
            }
        }
    }
}
