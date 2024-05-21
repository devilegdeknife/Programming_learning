
mod reverse_lists {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode {
                next: None,
                val,
            }
        }
    }

    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut current_node) = head {
            head = current_node.next.take();
            current_node.next = prev;
            prev = Some(current_node);
        }
        prev
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_reverse_list() {
            let node5 = ListNode::new(5);
            let mut node4 = ListNode::new(4);
            let mut node3 = ListNode::new(3);
            let mut node2 = ListNode::new(2);
            let mut node1 = ListNode::new(1);

            node4.next = Some(Box::new(node5));
            node3.next = Some(Box::new(node4));
            node2.next = Some(Box::new(node3));
            node1.next = Some(Box::new(node2));

            let reversed_node1 = ListNode::new(1);
            let mut reversed_node2 = ListNode::new(2);
            let mut reversed_node3 = ListNode::new(3);
            let mut reversed_node4 = ListNode::new(4);
            let mut reversed_node5 = ListNode::new(5);

            reversed_node2.next = Some(Box::new(reversed_node1));
            reversed_node3.next = Some(Box::new(reversed_node2));
            reversed_node4.next = Some(Box::new(reversed_node3));
            reversed_node5.next = Some(Box::new(reversed_node4));

            assert_eq!(reverse_list(Some(Box::new(node1))), Some(Box::new(reversed_node5)));
        }
    }

}