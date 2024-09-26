// Визначення структури ListNode
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut less_head = Some(Box::new(ListNode::new(0)));
    let mut greater_head = Some(Box::new(ListNode::new(0)));
    let mut less = &mut less_head;
    let mut greater = &mut greater_head;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();
        if node.val < x {
            less.as_mut().unwrap().next = Some(node);
            less = &mut less.as_mut().unwrap().next;
        } else {
            greater.as_mut().unwrap().next = Some(node);
            greater = &mut greater.as_mut().unwrap().next;
        }
    }

    less.as_mut().unwrap().next = greater_head.unwrap().next;
    less_head.unwrap().next
}

// Допоміжна функція для створення зв'язаного списку з вектора
pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut new_node = ListNode::new(val);
        new_node.next = head;
        head = Some(Box::new(new_node));
    }
    head
}

// Допоміжна функція для перетворення зв'язаного списку у вектор
pub fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

fn main() {
    let head = vec_to_list(vec![1, 4, 3, 2, 5, 2]);
    let x = 3;
    let partitioned_head = partition(head, x);
    let result = list_to_vec(partitioned_head);
    println!("{:?}", result); // [1, 2, 2, 4, 3, 5]

    let head = vec_to_list(vec![2, 1]);
    let x = 2;
    let partitioned_head = partition(head, x);
    let result = list_to_vec(partitioned_head);
    println!("{:?}", result); // [1, 2]
}

