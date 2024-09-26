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

fn reorder_list(head: &mut Option<Rc<RefCell<ListNode>>>) {
    if head.is_none() || head.as_ref().unwrap().borrow().next.is_none() {
        return;
    }

    let mut slow = head.clone();
    let mut fast = head.clone();
    while fast.as_ref().unwrap().borrow().next.is_some()
        && fast
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .is_some()
    {
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
    }

    let mut second_half = slow.as_ref().unwrap().borrow_mut().next.take();

    let mut prev = None;
    let mut curr = second_half;
    while let Some(current) = curr {
        let next = current.borrow_mut().next.take();
        current.borrow_mut().next = prev;
        prev = Some(current);
        curr = next;
    }
    second_half = prev;

    let mut first = head.clone();
    let mut second = second_half;
    while second.is_some() {
        let first_next = first.as_ref().unwrap().borrow_mut().next.take();
        let second_next = second.as_ref().unwrap().borrow_mut().next.take();

        first.as_ref().unwrap().borrow_mut().next = second.clone();
        if let Some(ref first_next_node) = first_next {
            second.as_ref().unwrap().borrow_mut().next = Some(first_next_node.clone());
        }

        first = first_next;
        second = second_next;
    }
}

fn create_linked_list(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let mut dummy = Some(Rc::new(RefCell::new(ListNode::new(0))));
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
    // Приклад 1
    let mut head1 = create_linked_list(vec![1, 2, 3, 4]);
    println!("Приклад 1:");
    println!("До: {:?}", linked_list_to_vec(head1.clone()));
    reorder_list(&mut head1);
    println!("Після: {:?}", linked_list_to_vec(head1));

    // Приклад 2
    let mut head2 = create_linked_list(vec![1, 2, 3, 4, 5]);
    println!("\nПриклад 2:");
    println!("До: {:?}", linked_list_to_vec(head2.clone()));
    reorder_list(&mut head2);
    println!("Після: {:?}", linked_list_to_vec(head2));
}
