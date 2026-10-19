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
        if (root) {
            q.push({root, 0});
        }
        vector<vector<int>> res;
        int last = -1;
        while (!q.empty()) {
            auto [root, level] = q.front(); q.pop();
            if (last != level) {
                res.push_back({root->val});
                last = level;
            } else {
                res[level].push_back(root->val);
            }
            if (root->left) {
                q.push({root->left, level+1});
            }
            if (root->right) {
                q.push({root->right, level+1});
            }
        }

        return res;
    }
};
