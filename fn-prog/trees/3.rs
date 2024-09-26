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

fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    match root {
        Some(mut node) => {
            let left = invert_tree(node.left.take());
            let right = invert_tree(node.right.take());
            node.left = right;
            node.right = left;
            Some(node)
        }
        None => None,
    }
}

fn main() {
    let mut root1 = Some(Box::new(TreeNode::new(4)));
    root1.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root1.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
    root1.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));
    root1.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
    root1.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(6)));
    root1.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(9)));

    let inverted_root1 = invert_tree(root1);
    println!("{:?}", inverted_root1); // Some(TreeNode { val: 4, left: Some(TreeNode { val: 7, left: Some(TreeNode { val: 9, left: None, right: None }), right: Some(TreeNode { val: 6, left: None, right: None }) }), right: Some(TreeNode { val: 2, left: Some(TreeNode { val: 3, left: None, right: None }), right: Some(TreeNode { val: 1, left: None, right: None }) }) })

    let mut root2 = Some(Box::new(TreeNode::new(2)));
    root2.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));
    root2.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));

    let inverted_root2 = invert_tree(root2);
    println!("{:?}", inverted_root2); // Some(TreeNode { val: 2, left: Some(TreeNode { val: 3, left: None, right: None }), right: Some(TreeNode { val: 1, left: None, right: None }) })

    let root3: Option<Box<TreeNode>> = None;

    let inverted_root3 = invert_tree(root3);
    println!("{:?}", inverted_root3); // None
}
