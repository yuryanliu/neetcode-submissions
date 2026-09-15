/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

class Solution {
public:
    int diameterOfBinaryTree(TreeNode* root) {
        int global = 0;
        diameterOfBinaryTreeHelper(root, global);
        return std::max(global-1, 0);                         
    }

    int diameterOfBinaryTreeHelper(TreeNode* root, int& global) {
        if (root == nullptr) {
            return 0;
        }
        int depth = std::max(1 + diameterOfBinaryTreeHelper(root->left, global),  
                             1 + diameterOfBinaryTreeHelper(root->right, global));
        int local = std::max(depth, 
                             1 + diameterOfBinaryTreeHelper(root->left, global) 
                               + diameterOfBinaryTreeHelper(root->right, global));
        global = std::max(global, local);
        return depth;
    }
};
