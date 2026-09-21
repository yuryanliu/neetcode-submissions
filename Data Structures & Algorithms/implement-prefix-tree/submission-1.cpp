struct Node {
    bool val;
    Node* next[26];

    Node() {
        val = false;
        for (int i = 0; i < 26; ++i) {
            next[i] = nullptr;
        }
    }
};

class PrefixTree {
private:
    Node* root = nullptr;

    Node* insert(Node* x, string& word, int d) {
        if (!x) x = new Node();
        if (d == word.size()) { x->val = true; return x; }
        int c = word[d] - 'a';
        x->next[c] = insert(x->next[c], word, d+1);
        return x;
    }
    
    Node* search(Node* x, string& word, int d) {
        if (!x) return nullptr;
        if (d == word.size()) return x;
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
        if (!x) return false;
        return x->val;
    }
    
    bool startsWith(string prefix) {
        Node* x = search(root, prefix, 0);
        if (!x) return false;
        return true;
    }
};
