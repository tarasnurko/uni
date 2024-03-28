use std::io;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref()?.next.is_none() {
        return head;
    }

    let mut first = head.unwrap();
    let mut second = first.next.take().unwrap();

    first.next = swap_pairs(second.next.take());
    second.next = Some(first);

    Some(second)
}

fn create_list(num_nodes: usize) -> Option<Box<ListNode>> {
    if num_nodes == 0 {
        return None;
    }

    let mut head = Some(Box::new(ListNode::new(1)));
    let mut tail = &mut head;
    for i in 2..=num_nodes {
        let node = Some(Box::new(ListNode::new(i as i32)));
        tail.as_mut().unwrap().next = node;
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn print_list(head: &Option<Box<ListNode>>) {
  let mut current = head;
  while let Some(node) = current {
      print!("{} ", node.val);
      if node.next.is_some() {
          print!("-> ");
      }
      current = &node.next;
  }
  println!();
}

fn main() {
    println!("Enter the number of nodes:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_nodes: usize = input.trim().parse().expect("Invalid input");

    let list = create_list(num_nodes);

    println!("Input: ");
    print_list(&list);
    println!("Output: ");
    print_list(&swap_pairs(list));
}
