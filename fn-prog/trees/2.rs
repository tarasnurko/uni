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

fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
    fn is_mirror(left: &Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> bool {
        match (left, right) {
            (Some(left_node), Some(right_node)) => {
                left_node.val == right_node.val
                    && is_mirror(&left_node.left, &right_node.right)
                    && is_mirror(&left_node.right, &right_node.left)
            }
            (None, None) => true,
            _ => false,
        }
    }

    if let Some(root_node) = root {
        is_mirror(&root_node.left, &root_node.right)
    } else {
        true
    }
}

fn main() {
    let mut root1 = Some(Box::new(TreeNode::new(1)));
    root1.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root1.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));
    root1.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
    root1.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
    root1.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root1.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));

    let result1 = is_symmetric(root1);
    println!("{}", result1); // true

    let mut root2 = Some(Box::new(TreeNode::new(1)));
    root2.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root2.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));
    root2.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
    root2.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));

    let result2 = is_symmetric(root2);
    println!("{}", result2); // false
}
