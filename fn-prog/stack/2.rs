#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn inorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    fn inorder(node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            inorder(&n.left, result);
            result.push(n.val);
            inorder(&n.right, result);
        }
    }

    let mut result = Vec::new();
    inorder(&root, &mut result);
    result
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
            right: None,
        })),
    }));
    println!("{:?}", inorder_traversal(root)); // [1, 3, 2]

    let root: Option<Box<TreeNode>> = None;
    println!("{:?}", inorder_traversal(root)); // []

    let root = Some(Box::new(TreeNode::new(1)));
    println!("{:?}", inorder_traversal(root)); // [1]
}
