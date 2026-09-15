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
    bool isBalanced(TreeNode* root) {
        if (root == nullptr) {
            return true;
        }
        return std::abs(maxHeight(root->left) - maxHeight(root->right)) <= 1 &&
                isBalanced(root->left) && isBalanced(root->right);
    }

    int maxHeight(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        return std::max(1 + maxHeight(root->left), 1 + maxHeight(root->right));
    }
};
