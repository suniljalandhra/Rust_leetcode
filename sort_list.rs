// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec: Vec<i32> = vec![];
        while let Some(mut node) = head { 
            vec.push(node.val);
            head = node.next;
        } 
        vec.sort(); 
        let mut next = None; 
        while let Some(val) = vec.pop() { 
            next = Some(Box::new(ListNode{next, val}))
        }
        next 
    }
}
