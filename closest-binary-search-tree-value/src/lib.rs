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
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        if let Some(ref rf) = root {
            let mut val = rf.borrow().val;
            let mut closet = val;
            let mut r = root;

            while let Some(ref node) = r {
                val = node.borrow().val;
                closet = if (val as f64 - target).abs() < (closet as f64 - target).abs() { val } else { closet };
                r = if target < val as f64 { node.borrow_mut().left.take() } else { node.borrow_mut().right.take() };
            }

            closet
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
