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

fn recover_from_preorder(traversal: String) -> Option<Box<TreeNode>> {
    fn parse_node(traversal: &Vec<char>, index: &mut usize, depth: usize) -> Option<Box<TreeNode>> {
        let mut current_depth = 0;
        while *index < traversal.len() && traversal[*index] == '-' {
            current_depth += 1;
            *index += 1;
        }

        if current_depth != depth {
            *index -= current_depth;
            return None;
        }

        let mut value = 0;
        while *index < traversal.len() && traversal[*index].is_digit(10) {
            value = value * 10 + (traversal[*index] as i32 - '0' as i32);
            *index += 1;
        }

        let mut node = Box::new(TreeNode::new(value));
        node.left = parse_node(traversal, index, depth + 1);
        node.right = parse_node(traversal, index, depth + 1);

        Some(node)
    }

    let traversal_chars: Vec<char> = traversal.chars().collect();
    let mut index = 0;
    parse_node(&traversal_chars, &mut index, 0)
}

fn main() {
    let traversal1 = "1-2--3--4-5--6--7".to_string();
    let root1 = recover_from_preorder(traversal1);
    println!("{:?}", root1); // Some(TreeNode { val: 1, left: Some(TreeNode { val: 2, left: Some(TreeNode { val: 3, left: None, right: None }), right: Some(TreeNode { val: 4, left: None, right: None }) }), right: Some(TreeNode { val: 5, left: Some(TreeNode { val: 6, left: None, right: None }), right: Some(TreeNode { val: 7, left: None, right: None }) }) })

    let traversal2 = "1-2--3---4-5--6---7".to_string();
    let root2 = recover_from_preorder(traversal2);
    println!("{:?}", root2); // Some(TreeNode { val: 1, left: Some(TreeNode { val: 2, left: Some(TreeNode { val: 3, left: Some(TreeNode { val: 4, left: None, right: None }), right: None }), right: None }), right: Some(TreeNode { val: 5, left: Some(TreeNode { val: 6, left: Some(TreeNode { val: 7, left: None, right: None }), right: None }), right: None }) })

    let traversal3 = "1-401--349---90--88".to_string();
    let root3 = recover_from_preorder(traversal3);
    println!("{:?}", root3); // Some(TreeNode { val: 1, left: Some(TreeNode { val: 401, left: Some(TreeNode { val: 349, left: Some(TreeNode { val: 90, left: None, right: None }), right: None }), right: Some(TreeNode { val: 88, left: None, right: None }) }), right: None })
}
