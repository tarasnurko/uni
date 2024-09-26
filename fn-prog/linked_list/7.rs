use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn merge_k_lists(mut lists: Vec<Option<Rc<RefCell<ListNode>>>>) -> Option<Rc<RefCell<ListNode>>> {
    let dummy = Rc::new(RefCell::new(ListNode::new(0)));
    let mut tail = dummy.clone();

    let mut heap: std::collections::BinaryHeap<(i32, usize)> = std::collections::BinaryHeap::new();

    for (i, list) in lists.iter().enumerate() {
        if let Some(node) = list {
            heap.push((-(node.borrow().val), i));
        }
    }

    while let Some((value, index)) = heap.pop() {
        let val = -value;
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        tail.borrow_mut().next = Some(new_node.clone());
        tail = new_node;

        if let Some(node) = lists[index].take() {
            if let Some(next) = node.borrow_mut().next.take() {
                heap.push((-(next.borrow().val), index));
                lists[index] = Some(next);
            }
        }
    }
    let result = dummy.borrow_mut().next.take();
    result
}

fn create_linked_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let mut dummy = Rc::new(RefCell::new(ListNode::new(0)));
    let mut current = dummy.clone();

    for val in values {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
    }

    let result = dummy.borrow_mut().next.take();
    result
}

fn linked_list_to_vec(head: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        result.push(node.borrow().val);
        current = node.borrow().next.clone();
    }
    result
}

fn main() {
    let lists = vec![
        create_linked_list(vec![1, 4, 5]),
        create_linked_list(vec![1, 3, 4]),
        create_linked_list(vec![2, 6]),
    ];

    let result = merge_k_lists(lists);
    println!("{:?}", linked_list_to_vec(result));
}
