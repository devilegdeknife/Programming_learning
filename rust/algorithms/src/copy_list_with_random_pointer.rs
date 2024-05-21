use std::collections::HashMap;

// Assuming the Node struct is defined as follows:
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
    pub random: Option<*mut Node>,
}

impl Node {
    #[inline]
    fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: None,
        }
    }
}

pub fn copy_random_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut node_map: HashMap<*const Node, *mut Node> = HashMap::new();
    let mut current = head.as_ref();
    let mut dummy = Box::new(Node::new(0));
    let mut prev = &mut dummy;

    // First pass: create new nodes and save the mapping
    while let Some(node) = current {
        let new_node = Box::into_raw(Box::new(Node::new(node.val)));
        node_map.insert(node.as_ref() as *const Node, new_node);
        prev.next = Some(unsafe { Box::from_raw(new_node.clone()) }); // Clone the raw pointer here
        prev = prev.next.as_mut().unwrap();
        current = node.next.as_ref();
    }

    // Second pass: set the random pointers of the new nodes
    current = head.as_ref();
    let mut new_current = dummy.next.as_mut();
    while let Some(node) = current {
        if let Some(new_node) = new_current {
            if let Some(random) = node.random {
                if let Some(&random_node) = node_map.get(&(random as *const _)) {
                    new_node.random = Some(random_node);
                }
            }
            new_current = new_node.next.as_mut();
        }
        current = node.next.as_ref();
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_random_list() {
        // 创建一个链表节点
        let mut node1 = Box::new(Node::new(1));
        let mut node2 = Box::new(Node::new(2));
        let mut node3 = Box::new(Node::new(3));

        // 设置next指针
        node1.next = Some(node2.clone());
        node2.next = Some(node3.clone());

        // 设置random指针
        node1.random = Some(&mut *node3);
        node2.random = Some(&mut *node1);
        node3.random = Some(&mut *node2);

        // 将头节点放入Option中
        let head = Some(node1);

        // 复制链表
        let copied_head = copy_random_list(head.clone());

        // 检查复制后的链表结构
        let mut original = head.as_ref();
        let mut copied = copied_head.as_ref();
        while let (Some(orig_node), Some(copied_node)) = (original, copied) {
            // 检查节点值
            assert_eq!(orig_node.val, copied_node.val);

            // 检查next指针
            assert_eq!(orig_node.next.as_ref().map(|node| node.val), copied_node.next.as_ref().map(|node| node.val));

            // 检查random指针
            if let (Some(orig_random), Some(copied_random)) = (orig_node.random, copied_node.random) {
                assert_eq!(unsafe { (*orig_random).val }, unsafe { (*copied_random).val });
            } else {
                assert!(orig_node.random.is_none() && copied_node.random.is_none());
            }

            // 移动到下一个节点
            original = orig_node.next.as_ref();
            copied = copied_node.next.as_ref();
        }
    }
}
