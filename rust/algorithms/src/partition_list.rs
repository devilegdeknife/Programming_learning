use std::mem;

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
}

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut before_head = Box::new(ListNode::new(0));
        let mut before = &mut before_head;
        let mut after_head = Box::new(ListNode::new(0));
        let mut after = &mut after_head;

        let mut current = head;
        while let Some(mut node) = current {
            current = mem::replace(&mut node.next, None); // 替换node.next的所有权
            if node.val < x {
                before.next = Some(node);
                before = before.next.as_mut().unwrap();
            } else {
                after.next = Some(node);
                after = after.next.as_mut().unwrap();
            }
        }

        after.next = None;
        before.next = after_head.next.take(); // 将after_head.next的所有权转移给before.next

        before_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_linked_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    #[test]
    fn test_all_nodes_less_than_x() {
        let head = create_linked_list(&[1, 2, 3, 4, 5]);
        let x = 6;
        let result = Solution::partition(head, x);
        let result_vec = linked_list_to_vec(result);
        assert_eq!(result_vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_all_nodes_greater_than_or_equal_to_x() {
        let head = create_linked_list(&[6, 7, 8, 9, 10]);
        let x = 1;
        let result = Solution::partition(head, x);
        let result_vec = linked_list_to_vec(result);
        assert_eq!(result_vec, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_mixed_nodes() {
        let head = create_linked_list(&[1, 4, 3, 2, 5, 2]);
        let x = 3;
        let result = Solution::partition(head, x);
        let result_vec = linked_list_to_vec(result);
        assert_eq!(result_vec, vec![1, 2, 2, 4, 3, 5]);
    }

    #[test]
    fn test_all_nodes_equal_to_x() {
        let head = create_linked_list(&[2, 2, 2, 2, 2]);
        let x = 2;
        let result = Solution::partition(head, x);
        let result_vec = linked_list_to_vec(result);
        assert_eq!(result_vec, vec![2, 2, 2, 2, 2]);
    }

    #[test]
    fn test_empty_list() {
        let head = create_linked_list(&[]);
        let x = 1;
        let result = Solution::partition(head, x);
        let result_vec = linked_list_to_vec(result);
        assert_eq!(result_vec, vec![]);
    }

    #[test]
    fn test_single_node_less_than_x() {
        let head = create_linked_list(&[1]);
        let x = 2;
        let result = Solution::partition(head, x);
        let result_vec = linked_list_to_vec(result);
        assert_eq!(result_vec, vec![1]);
    }

    #[test]
    fn test_single_node_greater_than_or_equal_to_x() {
        let head = create_linked_list(&[2]);
        let x = 2;
        let result = Solution::partition(head, x);
        let result_vec = linked_list_to_vec(result);
        assert_eq!(result_vec, vec![2]);
    }
}