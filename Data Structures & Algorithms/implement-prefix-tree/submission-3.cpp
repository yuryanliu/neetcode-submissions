struct Node {
    bool value;
    Node* next[26];

    Node() {
        value = false;
        for (int c = 0; c < 26; ++c) {
            next[c] = nullptr;
        }
    }
};

class PrefixTree {
private:
    Node* root = nullptr;
    
    Node* insert(Node* x, string& word, size_t d) {
        if (x == nullptr) { x = new Node(); }
        if (d == word.size()) { x->value = true; return x; }
        int c = word[d] - 'a';
        x->next[c] = insert(x->next[c], word, d+1);
        return x;
    }
    
    Node* search(Node* x, string& word, size_t d) {
        if (x == nullptr) { return nullptr; }
        if (d == word.size()) { return x; }
        int c = word[d] - 'a';
        return search(x->next[c], word, d+1);
    }

public:
    PrefixTree() {
        
    }
    
    void insert(string word) {
        root = insert(root, word, 0);
    }
    
    bool search(string word) {
        Node* x = search(root, word, 0);
        if (x == nullptr) return false;
        return x->value;
    }
    
    bool startsWith(string prefix) {
        Node* x = search(root, prefix, 0);
        if (x == nullptr) return false;
        return true;
    }
};
