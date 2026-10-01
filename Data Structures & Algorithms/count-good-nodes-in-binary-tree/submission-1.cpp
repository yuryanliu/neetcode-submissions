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
    int goodNodes(TreeNode* root) {
        if (root == nullptr) return 0;
        int count = 0;
        dfs(root, count, root->val);
        return count;
    }

    void dfs(TreeNode* root, int& count, int prev_max) {
        //if (root == nullptr) return;
        if (prev_max <= root->val) {
            prev_max = root->val;
            count++;
        }
        if (root->left != nullptr) {
            dfs(root->left, count, prev_max);
        }
        if (root->right != nullptr) {
            dfs(root->right, count, prev_max);
        }
    }
};
