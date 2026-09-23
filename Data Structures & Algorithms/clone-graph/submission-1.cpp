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
        return dfs(node, m);
    }

    Node* dfs(Node* node, unordered_map<Node*, Node*>& m) {
        if (node == nullptr) return nullptr;
        Node* new_node = new Node(node->val);
        m.insert({node, new_node});
        for (Node* neighbor : node->neighbors) {
            auto it = m.find(neighbor);
            if (it == m.end()) {
                new_node->neighbors.push_back(dfs(neighbor, m));
            } else {
                new_node->neighbors.push_back(it->second);
            }
        }
        return new_node;
    }
};
