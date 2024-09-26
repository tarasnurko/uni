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

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;
    let mut prev_val: Option<i32> = None;
    let mut new_head: Option<Box<ListNode>> = None;
    let mut tail = &mut new_head;

    while let Some(mut node) = current {
        current = node.next.take();
        if prev_val != Some(node.val) {
            prev_val = Some(node.val);
            tail = &mut tail.insert(node).next;
        }
    }

    new_head
}

fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    }));

    let result1 = delete_duplicates(list1);
    println!("Output: {:?}", result1); // [1, 2]

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        })),
    }));

    let result2 = delete_duplicates(list2);
    println!("Output: {:?}", result2); // [1, 2, 3]
}
