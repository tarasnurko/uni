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

impl Clone for TreeNode {
    fn clone(&self) -> Self {
        TreeNode {
            val: self.val,
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

fn serialize(root: Option<Box<TreeNode>>) -> String {
    fn helper(node: &Option<Box<TreeNode>>, result: &mut Vec<String>) {
        if let Some(n) = node {
            result.push(n.val.to_string());
            helper(&n.left, result);
            helper(&n.right, result);
        } else {
            result.push("null".to_string());
        }
    }

    let mut result = Vec::new();
    helper(&root, &mut result);
    result.join(",")
}

fn deserialize(data: String) -> Option<Box<TreeNode>> {
    fn helper(data: &mut std::vec::IntoIter<String>) -> Option<Box<TreeNode>> {
        if let Some(val) = data.next() {
            if val == "null" {
                return None;
            }
            let mut node = Box::new(TreeNode::new(val.parse().unwrap()));
            node.left = helper(data);
            node.right = helper(data);
            Some(node)
        } else {
            None
        }
    }

    let data_vec: Vec<String> = data.split(',').map(|s| s.to_string()).collect();
    let mut data_iter = data_vec.into_iter();
    helper(&mut data_iter)
}

fn main() {
    let mut root1 = Some(Box::new(TreeNode::new(1)));
    root1.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root1.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
    root1.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root1.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));

    let serialized = serialize(root1.clone());
    println!("{}", serialized); // "1,2,null,null,3,4,null,null,5,null,null"

    let deserialized = deserialize(serialized);
    println!("{:?}", deserialized); // Some(TreeNode { val: 1, left: Some(TreeNode { val: 2, left: None, right: None }), right: Some(TreeNode { val: 3, left: Some(TreeNode { val: 4, left: None, right: None }), right: Some(TreeNode { val: 5, left: None, right: None }) }) })
}
