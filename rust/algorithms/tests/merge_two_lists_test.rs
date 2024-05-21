use algorithms::merge_two_lists;
use algorithms::merge_two_lists::{list_to_vec, vec_to_list};
#[test]
fn test_merge_two_lists() {
    let l1 = vec_to_list(vec![1, 2, 4]);
    let l2 = vec_to_list(vec![1, 3, 4]);
    let merged = merge_two_lists::merge_two_lists(l1, l2);
    assert_eq!(list_to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
}

