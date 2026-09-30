/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        unordered_map<Node*, Node*> m;
        return copyRandomListHelper(head, m);
    }

    Node* copyRandomListHelper(Node* head, unordered_map<Node*, Node*>& m) {
        if (head == nullptr) return nullptr;    
        Node* root = new Node(head->val);
        m.insert({head, root});
        root->next = copyRandomListHelper(head->next, m);
        if (head->random != nullptr) {
            root->random = m[head->random];
        }
        return root;
    }
};
