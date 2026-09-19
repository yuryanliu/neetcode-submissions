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
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        return buildTreeHelper(preorder.begin(), preorder.end(),
                               inorder.begin(), inorder.end());
    }

    TreeNode* buildTreeHelper(vector<int>::iterator preorder_begin, 
                              vector<int>::iterator preorder_end,
                              vector<int>::iterator inorder_begin, 
                              vector<int>::iterator inorder_end) {
        if (preorder_begin >= preorder_end) return nullptr;
        
        TreeNode* root = new TreeNode(*preorder_begin);
        auto it = std::find(inorder_begin, inorder_end, root->val);
        int left_size = it - inorder_begin;

        root->left = buildTreeHelper(preorder_begin+1, preorder_begin+1+left_size, 
                                     inorder_begin, it);
        root->right = buildTreeHelper(preorder_begin+1+left_size, preorder_end, 
                                      it+1, inorder_end);
        return root;
    }
};
