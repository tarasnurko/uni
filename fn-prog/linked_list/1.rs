#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(mut l1), Some(mut l2)) => {
            if l1.val < l2.val {
                l1.next = merge_two_lists(l1.next.take(), Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_lists(Some(l1), l2.next.take());
                Some(l2)
            }
        }
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (None, None) => None,
    }
}

fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let merged_list = merge_two_lists(list1, list2);
    println!("Output: {:?}", merged_list); // [1, 1, 2, 3, 4, 4]

    let empty_list1 = None;
    let empty_list2 = None;
    let merged_empty = merge_two_lists(empty_list1, empty_list2);
    println!("Output: {:?}", merged_empty); // []

    let single_list = Some(Box::new(ListNode::new(0)));
    let merged_single = merge_two_lists(None, single_list);
    println!("Output: {:?}", merged_single); // [0]
}
