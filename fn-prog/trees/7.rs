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

fn min_camera_cover(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<TreeNode>>, cameras: &mut i32) -> i32 {
        if let Some(n) = node {
            let left = dfs(&n.left, cameras);
            let right = dfs(&n.right, cameras);

            if left == 0 || right == 0 {
                *cameras += 1;
                return 2;
            }
            if left == 2 || right == 2 {
                return 1;
            }
            return 0;
        }
        1
    }

    let mut cameras = 0;
    if dfs(&root, &mut cameras) == 0 {
        cameras += 1;
    }
    cameras
}

fn main() {
    let mut root1 = Some(Box::new(TreeNode::new(0)));
    root1.as_mut().unwrap().left = Some(Box::new(TreeNode::new(0)));
    root1.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(0)));
    root1.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(0)));

    let result1 = min_camera_cover(root1);
    println!("{}", result1); // 1

    let mut root2 = Some(Box::new(TreeNode::new(0)));
    root2.as_mut().unwrap().left = Some(Box::new(TreeNode::new(0)));
    root2.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(0)));
    root2.as_mut().unwrap().left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(0)));

    let result2 = min_camera_cover(root2);
    println!("{}", result2); // 2
}
