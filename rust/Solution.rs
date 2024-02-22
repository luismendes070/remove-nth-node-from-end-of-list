// Copilot

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let mut slow = &mut dummy;
        let mut fast = &dummy;

        // Move fast pointer n steps ahead
        for _ in 0..=n {
            fast = &fast.as_ref().unwrap().next;
        }

        // Move both pointers until fast reaches the end
        while fast.is_some() {
            slow = &mut slow.as_mut().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
        }

        // Remove the nth node from the end
        let next = slow.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = next.and_then(|node| node.next);

        dummy.unwrap().next
    }
}

fn main() {

        // Create an empty linked list
        let mut list = Solution::new();

}
