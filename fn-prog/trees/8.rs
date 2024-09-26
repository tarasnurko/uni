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

fn vertical_traversal(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    fn dfs(node: &Option<Box<TreeNode>>, row: i32, col: i32, nodes: &mut Vec<(i32, i32, i32)>) {
        if let Some(n) = node {
            nodes.push((col, row, n.val));
            dfs(&n.left, row + 1, col - 1, nodes);
            dfs(&n.right, row + 1, col + 1, nodes);
        }
    }

    let mut nodes = Vec::new();
    dfs(&root, 0, 0, &mut nodes);

    nodes.sort();

    let mut result = Vec::new();
    let mut last_col = nodes[0].0;
    let mut col_vals = Vec::new();

    for (col, _row, val) in nodes {
        if col != last_col {
            result.push(col_vals);
            col_vals = Vec::new();
            last_col = col;
        }
        col_vals.push(val);
    }
    result.push(col_vals);

    result
}

fn main() {
    let mut root1 = Some(Box::new(TreeNode::new(3)));
    root1.as_mut().unwrap().left = Some(Box::new(TreeNode::new(9)));
    root1.as_mut().unwrap().right = Some(Box::new(TreeNode::new(20)));
    root1.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root1.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    let result1 = vertical_traversal(root1);
    println!("{:?}", result1); // [[9], [3, 15], [20], [7]]

    let mut root2 = Some(Box::new(TreeNode::new(1)));
    root2.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root2.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
    root2.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root2.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
    root2.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(6)));
    root2.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    let result2 = vertical_traversal(root2);
    println!("{:?}", result2); // [[4], [2], [1, 5, 6], [3], [7]]
}
