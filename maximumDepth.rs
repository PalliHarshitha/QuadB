use std::io;

// Definition for a binary tree node.
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode { val: value, left: None, right: None }
    }
}

// Define a function to calculate the maximum depth of the binary tree

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => { // If the root node exists
            // Recursively calculate the depth of the left and right subtrees
            let left_depth = max_depth(&node.left); // Depth of the left subtree
            let right_depth = max_depth(&node.right); // Depth of the right subtree
            // Return the maximum depth of the left and right subtrees, plus 1 (for the current node)
            left_depth.max(right_depth) + 1
        }
    }
}

// Function for building a binary tree
fn build_tree() -> Option<Box<TreeNode>> {
    let mut input = String::new();
    println!("Enter the value for the root node:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let root_val: i32 = input.trim().parse().expect("Invalid input");
    let mut root = Some(Box::new(TreeNode::new(root_val)));
    let mut queue = vec![root.as_mut().unwrap()];

    while let Some(current_node) = queue.pop() {
        let mut input = String::new();
        println!("Enter the left child value for node {} (or type 'None'):", current_node.val);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let left_val = input.trim();
        if left_val.to_lowercase() != "none" {
            let left_val: i32 = left_val.parse().expect("Invalid input");
            current_node.left = Some(Box::new(TreeNode::new(left_val)));
            queue.push(current_node.left.as_mut().unwrap());
        }

        let mut input = String::new();
        println!("Enter the right child value for node {} (or type 'None'):", current_node.val);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let right_val = input.trim();
        if right_val.to_lowercase() != "none" {
            let right_val: i32 = right_val.parse().expect("Invalid input");
            current_node.right = Some(Box::new(TreeNode::new(right_val)));
            queue.push(current_node.right.as_mut().unwrap());
        }
    }

    root
}

fn main() {
    let root = build_tree();
    println!("Maximum depth of the tree: {}", max_depth(&root));
}
