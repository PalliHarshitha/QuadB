class TreeNode:
    def __init__(self, value):
        self.val = value # Value of the current node
        self.left = None # Pointer to the left child node
        self.right = None # Pointer to the right child node

# Define a function to calculate the maximum depth of the binary tree
def max_depth(root):
    if root is None:
        return 0
    else:
        # Recursive case: calculate the maximum depth of the left and right subtrees
        left_depth = max_depth(root.left)
        right_depth = max_depth(root.right)
        # The maximum depth is the sum of the depths of the left and right subtrees plus one for the current node
        return max(left_depth, right_depth) + 1

# Function to build the binary tree based on user input
def build_tree():
    root_val = int(input("Enter the value for the root node: "))
    root = TreeNode(root_val) # Create the root node
    queue = [root] # Queue to process nodes

    while queue:
        current_node = queue.pop(0) # Process the first node in the queue

        # Get the values for the left and right children from the user
        left_val = input(f"Enter the left child value for node {current_node.val} (or type 'None'): ")
        if left_val.lower() != 'none':
            current_node.left = TreeNode(int(left_val))
            queue.append(current_node.left)

        right_val = input(f"Enter the right child value for node {current_node.val} (or type 'None'): ")
        if right_val.lower() != 'none':
            current_node.right = TreeNode(int(right_val))
            queue.append(current_node.right)

    return root

# Example usage:
root = build_tree()
print("Maximum depth of the tree:", max_depth(root))
