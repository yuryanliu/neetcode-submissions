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
        return kthSmallestHelper(root, k)->val;
    }

    TreeNode* kthSmallestHelper(TreeNode* root, int& k) {
        if (root == nullptr) return nullptr;
        if (root->left) {
            TreeNode* left = kthSmallestHelper(root->left, k);
            if (k == 0) return left;
        }
        k--;
        if (k==0) return root;
        if (root->right) {
            TreeNode* right = kthSmallestHelper(root->right, k);
            if (k == 0) return right;
        }
        return nullptr;
    }
};
