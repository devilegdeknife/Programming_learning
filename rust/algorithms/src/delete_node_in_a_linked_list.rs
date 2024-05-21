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
}

pub struct Solution;

impl Solution {
    pub fn delete_node(node: &mut ListNode) {
        // Since we cannot access the previous node, we can only delete the current node by
        // copying the next node's value and then skipping the next node.
        *node = match node.next.take() {
            Some(next_node) => *next_node,
            None => panic!("Cannot delete the last node in the list"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_node() {
        // Create a linked list: 4 -> 5 -> 1 -> 9
        let mut list = ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode::new(9))),
                })),
            })),
        };

        // Find the node with value 1
        let mut node_to_delete = &mut list;
        while node_to_delete.val != 1 {
            node_to_delete = node_to_delete.next.as_deref_mut().unwrap();
        }

        // Delete the node
        Solution::delete_node(node_to_delete);

        // Verify the result: 4 -> 5 -> 9
        assert_eq!(list.val, 4);
        assert_eq!(list.next.as_ref().unwrap().val, 5);
        assert_eq!(list.next.as_ref().unwrap().next.as_ref().unwrap().val, 9);
        assert!(list.next.as_ref().unwrap().next.as_ref().unwrap().next.is_none());
    }
}