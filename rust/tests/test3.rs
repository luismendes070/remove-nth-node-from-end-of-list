#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    struct ListNode {
        val: i32,
        next: Option<Box<ListNode>>,
    }

    impl ListNode {
        fn new(val: i32) -> Self {
            ListNode { val, next: None }
        }
    }

    use crate::solution::Solution;

    #[test]
    fn test_remove_nth_from_end() {
        // Test Case: [1,2,3,4,5], n = 2
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut current = head.as_mut();
        for val in 2..=5 {
            current.unwrap().next = Some(Box::new(ListNode::new(val)));
            current = current.unwrap().next.as_mut();
        }
        let modified_head = Solution::remove_nth_from_end(head, 2);
        let mut result = Vec::new();
        let mut current = modified_head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        assert_eq!(result, vec![1, 2, 3, 5]);

        // Test Case: [1], n = 1
        let head = Some(Box::new(ListNode::new(1)));
        let modified_head = Solution::remove_nth_from_end(head, 1);
        assert_eq!(modified_head, None);

        // Test Case: [1,2], n = 1
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        let modified_head = Solution::remove_nth_from_end(head, 1);
        let mut result = Vec::new();
        let mut current = modified_head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        assert_eq!(result, vec![1]);
    }
}
