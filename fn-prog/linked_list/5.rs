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

fn delete_node(node: &mut Option<Rc<RefCell<ListNode>>>) {
    if let Some(current_node) = node {
        let next_node = current_node.borrow_mut().next.take();
        if let Some(next) = next_node {
            let next_val = next.borrow().val;
            current_node.borrow_mut().val = next_val;
            current_node.borrow_mut().next = next.borrow_mut().next.take();
        }
    }
}

fn create_linked_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let dummy = Some(Rc::new(RefCell::new(ListNode::new(0))));
    let mut curr = dummy.clone();

    for &val in values.iter() {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        curr.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
        curr = Some(new_node);
    }

    dummy.unwrap().borrow_mut().next.take()
}

fn linked_list_to_vec(head: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = head;
    while let Some(node) = curr {
        result.push(node.borrow().val);
        curr = node.borrow().next.clone();
    }
    result
}

fn main() {
    let head1 = create_linked_list(vec![4, 5, 1, 9]);
    let mut node_to_delete = head1.as_ref().unwrap().borrow().next.clone();
    println!("Приклад 1: {:?}", linked_list_to_vec(head1.clone()));
    delete_node(&mut node_to_delete);
    println!("{:?}", linked_list_to_vec(head1));

    let head2 = create_linked_list(vec![4, 5, 1, 9]);
    let mut node_to_delete = head2
        .as_ref()
        .unwrap()
        .borrow()
        .next
        .as_ref()
        .unwrap()
        .borrow()
        .next
        .clone();
    println!("Приклад 2: {:?}", linked_list_to_vec(head2.clone()));
    delete_node(&mut node_to_delete);
    println!("{:?}", linked_list_to_vec(head2));
}
