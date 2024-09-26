use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut slow = head.clone();
    let mut fast = head;

    while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some() {
        slow = slow.unwrap().borrow().next.clone();
        fast = fast
            .unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .clone();

        if let (Some(s), Some(f)) = (&slow, &fast) {
            if Rc::ptr_eq(s, f) {
                return true;
            }
        }
    }

    false
}

fn create_linked_list(values: Vec<i32>, pos: i32) -> Option<Rc<RefCell<ListNode>>> {
    if values.is_empty() {
        return None;
    }

    let mut nodes = Vec::new();
    for &val in &values {
        nodes.push(Rc::new(RefCell::new(ListNode::new(val))));
    }

    for i in 0..nodes.len() - 1 {
        nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
    }

    if pos >= 0 && pos < values.len() as i32 {
        let last = nodes.last().unwrap();
        last.borrow_mut().next = Some(nodes[pos as usize].clone());
    }

    Some(nodes[0].clone())
}

fn main() {
    // Приклад 1
    let head1 = create_linked_list(vec![3, 2, 0, -4], 1);
    println!("Приклад 1: {}", has_cycle(head1)); // true

    // Приклад 2
    let head2 = create_linked_list(vec![1, 2], 0);
    println!("Приклад 2: {}", has_cycle(head2)); // true

    // Приклад 3
    let head3 = create_linked_list(vec![1], -1);
    println!("Приклад 3: {}", has_cycle(head3)); // false
}
