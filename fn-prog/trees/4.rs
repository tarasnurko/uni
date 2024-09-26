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

fn kth_smallest(root: Option<Box<TreeNode>>, k: i32) -> i32 {
    fn inorder_traversal(node: &Option<Box<TreeNode>>, elements: &mut Vec<i32>) {
        if let Some(n) = node {
            inorder_traversal(&n.left, elements);
            elements.push(n.val);
            inorder_traversal(&n.right, elements);
        }
    }

    let mut elements = Vec::new();
    inorder_traversal(&root, &mut elements);
    elements[(k - 1) as usize]
}

fn main() {
    let mut root1 = Some(Box::new(TreeNode::new(3)));
    root1.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));
    root1.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
    root1.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));

    let k1 = 1;
    let result1 = kth_smallest(root1, k1);
    println!("{}", result1); // 1

    let mut root2 = Some(Box::new(TreeNode::new(5)));
    root2.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
    root2.as_mut().unwrap().right = Some(Box::new(TreeNode::new(6)));
    root2.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root2.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
    root2.as_mut().unwrap().left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));

    let k2 = 3;
    let result2 = kth_smallest(root2, k2);
    println!("{}", result2); // 3
}
