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
    vector<vector<int>> levelOrder(TreeNode* root) {
        queue<pair<TreeNode*, int>> q;
        int last = -1;
        if (root != nullptr) {
            q.push({root, 0});
        }
        vector<vector<int>> res;
        while (!q.empty()) {
            auto p = q.front(); q.pop();
            if (last != p.second) {
                last = p.second;
                res.push_back({});
            }
            res.back().push_back(p.first->val);
            if (p.first->left != nullptr) {
                q.push({p.first->left, p.second + 1});
            }
            if (p.first->right != nullptr) {
                q.push({p.first->right, p.second + 1});
            }
        }
        return res;
    }
};
