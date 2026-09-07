/* 124. Binary Tree Maximum Path Sum */
/* Runtime: 0ms, Memory: 4.35MB */

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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::max_gain(&root, &mut max_sum);
        max_sum
    }

    // Returns the maximum downward path sum starting at 'node',
    // which is the value that can be extended upward to its parent.
    fn max_gain(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n_borrowed = n.borrow();

                // Discard negative gains from children(treat as 0)
                let left_gain = Self::max_gain(&n_borrowed.left, max_sum).max(0); 
                let right_gain = Self::max_gain(&n_borrowed.right, max_sum).max(0); 

                // Best path sum with this node as the turning point
                let price_new_path = n_borrowed.val + left_gain + right_gain;
                *max_sum = (*max_sum).max(price_new_path);

                // Only one side can be carried upward to the parent
                n_borrowed.val + left_gain.max(right_gain)
            }
        }
    }
}
