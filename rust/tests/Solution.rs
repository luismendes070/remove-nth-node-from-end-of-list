// ChatGPT
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let (mut fast, mut slow) = (&mut dummy, &mut dummy);

        // Move fast pointer n steps ahead
        for _ in 0..n {
            fast = &mut fast.as_mut().unwrap().next;
        }

        // Move fast and slow pointers simultaneously
        while fast.as_ref().unwrap().next.is_some() {
            fast = &mut fast.as_mut().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        // Remove the nth node
        let removed_node = slow.as_mut().unwrap().next.take();

        // Link the previous node with the next node
        if let Some(ref mut node) = slow {
            node.next = removed_node.unwrap().next.take();
        }

        dummy.unwrap().next
    }
}
