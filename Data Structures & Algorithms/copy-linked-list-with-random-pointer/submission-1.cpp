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
        Node* root = copyRandomListHelper(head, m);
        Node* p = head;
        Node* q = root;
        while (p != nullptr) {
            Node* random = p->random;
            if (random != nullptr) {
                q->random = m[random];
            }
            p = p->next;
            q = q->next;
        }

        return root;
    }

    Node* copyRandomListHelper(Node* head, unordered_map<Node*, Node*>& m) {
        if (head == nullptr) return nullptr;    
        Node* root = new Node(head->val);
        root->next = copyRandomListHelper(head->next, m);
        m.insert({head, root});
        return root;
    }
};
