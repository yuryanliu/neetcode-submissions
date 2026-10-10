/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* parent;
};
*/

/*
Solution: HashMap: 
Time: O(tree_depth)
Space: O(tree_depth)



*/

class Solution {
public:
    Node* lowestCommonAncestor(Node* p, Node * q) {
        unordered_map<int, Node*> m;
        while (p != nullptr) {
            //All Node.val are unique.
            m[p->val] = p;
            p = p->parent;
        }
        while (q != nullptr) {
            auto it = m.find(q->val);
            if (it != m.end()) {
                return it->second;
            }
            q = q->parent;
        }
        return nullptr;
    }
};