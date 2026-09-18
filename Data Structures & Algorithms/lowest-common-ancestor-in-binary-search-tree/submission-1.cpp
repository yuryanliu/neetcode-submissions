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
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (!root) return root;
        if (p && q && p->val > q->val) {
            return lowestCommonAncestor(root, q, p);
        }
        if (root->val == p->val || root->val == q->val || (p->val < root->val && root->val < q->val)) {
            return root;
        }
        if (q->val < root->val) {
            return lowestCommonAncestor(root->left, q, p);
        } else {
            return lowestCommonAncestor(root->right, q, p);
        }
    }
};
