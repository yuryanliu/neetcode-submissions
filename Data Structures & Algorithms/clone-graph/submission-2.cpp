/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
*/

class Solution {
public:
    Node* cloneGraph(Node* node) {
        unordered_map<Node*, Node*> m;
        return cloneGraphHelper(node, m);    
    }

    Node* cloneGraphHelper(Node* node, unordered_map<Node*, Node*>& m) {
        if (node == nullptr) return nullptr;
        Node* root = new Node(node->val);
        m[node] = root;
        for (auto neighbor : node->neighbors) {
            auto it = m.find(neighbor);
            if (it == m.end()) {
                root->neighbors.push_back(cloneGraphHelper(neighbor, m));
            } else {
                root->neighbors.push_back(it->second);
            }
        }
        return root;
    }
};
