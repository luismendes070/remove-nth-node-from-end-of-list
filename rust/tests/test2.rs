// In your test file, e.g., tests/test2.rs

// Import the ListNode type from Solution.rs
mod Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        // Create a linked list: 1 -> 2 -> 3 -> 4 -> 5
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

        // Remove the second node from the end
        let new_head = ListNode::remove_nth_from_end(head, 2);

        // Check if the new list is [1, 2, 3, 5]
        assert_eq!(new_head.unwrap().val, 1);
        assert_eq!(new_head.unwrap().next.as_ref().unwrap().val, 2);
        assert_eq!(new_head.unwrap().next.as_ref().unwrap().next.as_ref().unwrap().val, 3);
        assert_eq!(new_head.unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().val, 5);
        assert!(new_head.unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next.is_none());
    }

    // Add more test cases here to cover other scenarios
}
