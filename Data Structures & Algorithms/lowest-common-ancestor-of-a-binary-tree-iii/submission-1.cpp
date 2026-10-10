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
        Node* ptr1 = p;
        Node* ptr2 = q;
        while (ptr1 != ptr2) {
            ptr1 = ptr1->parent != nullptr ? ptr1->parent : q;
            ptr2 = ptr2->parent != nullptr ? ptr2->parent : p;
        }
        return ptr1;
    }
};