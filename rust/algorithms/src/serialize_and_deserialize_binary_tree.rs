use std::str::FromStr;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

type TreeNode = Rc<RefCell<Option<Box<Node>>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub left: TreeNode,
    pub right: TreeNode,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
        }
    }
}

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: TreeNode) -> String {
        let mut result = String::new();
        self._serialize(root, &mut result);
        result
    }

    fn _serialize(&self, node: TreeNode, result: &mut String) {
        match &*node.borrow() {
            Some(n) => {
                result.push_str(&n.val.to_string());
                result.push(',');
                self._serialize(n.left.clone(), result);
                self._serialize(n.right.clone(), result);
            }
            None => {
                result.push_str("null,");
            }
        }
    }

    pub fn deserialize(&self, data: String) -> TreeNode {
        let mut nodes: VecDeque<_> = data.split(',').collect();
        self._deserialize(&mut nodes)
    }

    fn _deserialize(&self, nodes: &mut VecDeque<&str>) -> TreeNode {
        if let Some(val) = nodes.pop_front() {
            if val == "null" {
                Rc::new(RefCell::new(None))
            } else {
                let mut node = Node::new(i32::from_str(val).unwrap());
                node.left = self._deserialize(nodes);
                node.right = self._deserialize(nodes);
                Rc::new(RefCell::new(Some(Box::new(node))))
            }
        } else {
            Rc::new(RefCell::new(None))
        }
    }
}
