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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head.unwrap();
        let mut fast = head.clone();
        let mut slow = head.as_mut();
        
        let mut meet_end =false;
        for i in 0..n{
            if fast.next.is_none() {
                meet_end = true;
                break;
            }
            fast = fast.next.unwrap();
            
        }
        if meet_end {
            if n == 1{
                return None;
            }
            return head.next;
        }
        while let Some(node) = fast.next {
            fast = node;
            slow = slow.next.as_mut().unwrap();
        }
        slow.next = slow.next.as_mut().unwrap().next.clone();
        Some(head)
    }
}
