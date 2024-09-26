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

fn max_path_sum(root: Option<Box<TreeNode>>) -> i32 {
    fn helper(node: &Option<Box<TreeNode>>, max_sum: &mut i32) -> i32 {
        if let Some(n) = node {
            let left_gain = helper(&n.left, max_sum).max(0);
            let right_gain = helper(&n.right, max_sum).max(0);
            let price_newpath = n.val + left_gain + right_gain;
            *max_sum = (*max_sum).max(price_newpath);
            n.val + left_gain.max(right_gain)
        } else {
            0
        }
    }

    let mut max_sum = i32::MIN;
    helper(&root, &mut max_sum);
    max_sum
}

fn main() {
    let mut root1 = Some(Box::new(TreeNode::new(1)));
    root1.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root1.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));

    let result1 = max_path_sum(root1);
    println!("{}", result1); // 6

    let mut root2 = Some(Box::new(TreeNode::new(-10)));
    root2.as_mut().unwrap().left = Some(Box::new(TreeNode::new(9)));
    root2.as_mut().unwrap().right = Some(Box::new(TreeNode::new(20)));
    root2.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root2.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    let result2 = max_path_sum(root2);
    println!("{}", result2); // 42
}
