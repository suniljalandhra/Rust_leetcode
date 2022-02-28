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
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    
    pub fn merge_trees(root1: Option<Node>, root2: Option<Node>) -> Option<Node> {
        fn recursive(n1: &Option<Node>,n2: &Option<Node>) -> Option<Node>{
            match (n1, n2) {
                (Some(n1), Some(n2)) => {
                    let (n1,n2) = (n1.borrow(), n2.borrow());
                    let mut root = TreeNode::new(n1.val + n2.val);
                    root.left = recursive(&n1.left, &n2.left);
                    root.right = recursive(&n1.right, &n2.right);
                    Some(Rc::new(RefCell::new(root)))
                }
                (None, Some(n2)) => Some(n2.clone()),
                (Some(n1), None) => Some(n1.clone()),
                (None,None) => None,
            }
        }
        recursive(&root1, &root2)
        
    }
}
