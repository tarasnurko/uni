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

fn double_number(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if head.is_none() {
        return None;
    }

    let mut prev = None;
    let mut current = head;
    while let Some(curr_node) = current {
        let next_temp = curr_node.borrow_mut().next.take();
        curr_node.borrow_mut().next = prev;
        prev = Some(curr_node);
        current = next_temp;
    }

    let mut carry = 0;
    let mut current = prev;
    let mut new_head = None;
    while let Some(curr_node) = current {
        let mut new_val = curr_node.borrow().val * 2 + carry;
        carry = new_val / 10;
        new_val %= 10;

        let new_node = Rc::new(RefCell::new(ListNode::new(new_val)));
        new_node.borrow_mut().next = new_head;
        new_head = Some(new_node);

        current = curr_node.borrow().next.clone();
    }

    if carry > 0 {
        let new_node = Rc::new(RefCell::new(ListNode::new(carry)));
        new_node.borrow_mut().next = new_head;
        new_head = Some(new_node);
    }

    new_head
}

fn create_linked_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let dummy = Rc::new(RefCell::new(ListNode::new(0)));
    let mut current = Some(dummy.clone());

    for &val in values.iter() {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        if let Some(curr) = current {
            curr.borrow_mut().next = Some(new_node.clone());
            current = Some(new_node);
        }
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
    // Приклад 1
    let head1 = create_linked_list(vec![1, 8, 9]);
    println!("Приклад 1: {:?}", linked_list_to_vec(head1.clone()));
    let result1 = double_number(head1);
    println!("{:?}", linked_list_to_vec(result1));

    // Приклад 2
    let head2 = create_linked_list(vec![9, 9, 9]);
    println!("Приклад 2: {:?}", linked_list_to_vec(head2.clone()));
    let result2 = double_number(head2);
    println!("{:?}", linked_list_to_vec(result2));
}
