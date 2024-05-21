// Definition for singly-linked list.
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

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(node1), Some(node2)) => {
            if node1.val < node2.val {
                Some(Box::new(ListNode {
                    val: node1.val,
                    next: merge_two_lists(node1.next, Some(node2)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: node2.val,
                    next: merge_two_lists(Some(node1), node2.next),
                }))
            }
        }
        (Some(node), None) | (None, Some(node)) => Some(node),
        _ => None,
    }
}

pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = list;
    while let Some(mut node) = current {
        vec.push(node.val);
        current = node.next.take();
    }
    vec
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 3 {
//         eprintln!("Please provide two lists of integers.");
//         return;
//     }
//
//     let l1: Vec<i32> = args[1]
//         .split(',')
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect();
//
//     let l2: Vec<i32> = args[2]
//         .split(',')
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect();
//
//     let l1 = vec_to_list(l1);
//     let l2 = vec_to_list(l2);
//     let merged = merge_two_lists(l1, l2);
//     let output = list_to_vec(merged);
//     println!("{:?}", output);
// }
