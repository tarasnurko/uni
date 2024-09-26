#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn is_same_tree(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
    match (p, q) {
        (Some(p_node), Some(q_node)) => {
            p_node.val == q_node.val
                && is_same_tree(p_node.left, q_node.left)
                && is_same_tree(p_node.right, q_node.right)
        }
        (None, None) => true,
        _ => false,
    }
}

fn main() {
    let mut p = Some(Box::new(TreeNode::new(1)));
    p.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    p.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));

    let mut q = Some(Box::new(TreeNode::new(1)));
    q.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    q.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));

    let result1 = is_same_tree(p, q);
    println!("{}", result1); // true

    let mut p = Some(Box::new(TreeNode::new(1)));
    p.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));

    let mut q = Some(Box::new(TreeNode::new(1)));
    q.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));

    let result2 = is_same_tree(p, q);
    println!("{}", result2); // false

    let mut p = Some(Box::new(TreeNode::new(1)));
    p.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    p.as_mut().unwrap().right = Some(Box::new(TreeNode::new(1)));

    let mut q = Some(Box::new(TreeNode::new(1)));
    q.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));
    q.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));

    let result3 = is_same_tree(p, q);
    println!("{}", result3); // false
}
