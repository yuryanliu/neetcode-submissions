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
    int kthSmallest(TreeNode* root, int k) {
        int cur = 0;
        return kthSmallestHelper(root, k, cur)->val;
    }

    TreeNode* kthSmallestHelper(TreeNode* root, int k, int& cur) {
        if (!root) return nullptr;
        
        TreeNode* left = kthSmallestHelper(root->left, k, cur);
        if (left != nullptr) return left;
        if (++cur == k) return root;
        TreeNode* right = kthSmallestHelper(root->right, k, cur);
        if (right != nullptr) return right;

        return nullptr;
    }
};
