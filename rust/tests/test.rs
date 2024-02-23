pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut len = 0;
    let mut first = dummy.as_ref();
    while let Some(node) = first {
        len += 1;
        first = node.next.as_ref();
    }
    len -= n;
    let mut first = dummy.as_mut();
    while len > 0 {
        if let Some(node) = first {
            first = node.next.as_mut();
        }
        len -= 1;
    }
    if let Some(node) = first {
        node.next = node.next.as_mut().unwrap().next.take();
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let mut node1 = Box::new(ListNode::new(1));
        let mut node2 = Box::new(ListNode::new(2));
        let node3 = Box::new(ListNode::new(3));
        node2.next = Some(node3);
        node1.next = Some(node2);
        let result = remove_nth_from_end(Some(node1), 2);
        assert_eq!(result.unwrap().val, 1);
    }
}