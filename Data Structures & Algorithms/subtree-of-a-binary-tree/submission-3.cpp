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
    bool isSubtree(TreeNode* root, TreeNode* subRoot) {
        return isSameTree(root, subRoot) || 
               (root != nullptr && (isSubtree(root->left, subRoot) || isSubtree(root->right, subRoot)));
    }

    bool isSameTree(TreeNode* p, TreeNode* q) {
        if ((!p && q) || (p && !q) || (p && q && p->val != q->val)) {
            return false;
        } else if (!p && !q) {
            return true;
        }

        return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
    }
};
