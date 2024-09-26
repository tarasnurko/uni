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

fn reverse_k_group(head: Option<Rc<RefCell<ListNode>>>, k: i32) -> Option<Rc<RefCell<ListNode>>> {
    if k <= 1 {
        return head;
    }

    let dummy = Rc::new(RefCell::new(ListNode::new(0)));
    dummy.borrow_mut().next = head;
    let mut prev = dummy.clone();

    loop {
        let mut start = prev.clone();
        let mut end = prev.clone();

        let mut count = 0;
        {
            let mut current = end.clone();
            while count < k {
                let next = current.borrow().next.clone();
                if let Some(next_node) = next {
                    current = next_node;
                    count += 1;
                } else {
                    break;
                }
            }
            if count == k {
                end = current;
            }
        }

        if count < k {
            break;
        }

        let next_group = end.borrow().next.clone();
        let mut current = start.borrow_mut().next.take();
        let mut prev_node = next_group.clone();
        for _ in 0..k {
            let next_node = current.as_ref().unwrap().borrow_mut().next.take();
            current.as_ref().unwrap().borrow_mut().next = prev_node;
            prev_node = current;
            current = next_node;
        }

        start.borrow_mut().next = prev_node;

        prev = start.borrow().next.as_ref().unwrap().clone();
    }

    let result = dummy.borrow_mut().next.take();
    result
}

fn main() {
    let values = vec![1, 2, 3, 4, 5];
    let k = 2;
    let head = create_linked_list(values);

    println!("Original list:");
    println!("{:?}", linked_list_to_vec(head.clone()));

    let reversed_head = reverse_k_group(head, k);

    println!("Reversed in groups of {}:", k);
    println!("{:?}", linked_list_to_vec(reversed_head));

    let values = vec![1, 2, 3, 4, 5];
    let k = 3;
    let head = create_linked_list(values);

    println!("Original list:");
    println!("{:?}", linked_list_to_vec(head.clone()));

    let reversed_head = reverse_k_group(head, k);

    println!("Reversed in groups of {}:", k);
    println!("{:?}", linked_list_to_vec(reversed_head));
}
