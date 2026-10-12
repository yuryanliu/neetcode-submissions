struct Node {
    bool value;
    Node* next[26];

    Node() {
        this->value = false;
        for (int ch = 0; ch < 26; ++ch) {
            this->next[ch] = nullptr;
        }
    }
};

class WordDictionary {
private:
    Node* root = nullptr;

    Node* addWord(Node* x, string& word, size_t d) {
        if (x == nullptr) { x = new Node(); }
        if (d == word.size()) { x->value = true; return x; }
        int ch = word[d] - 'a';
        x->next[ch] = addWord(x->next[ch], word, d+1);
        return x;
    }
    
    Node* search(Node* x, string& word, size_t d) {
        if (x == nullptr) { return nullptr; }
        if (d == word.size()) { return x; }
        if (word[d] == '.') {
            for (int ch = 0; ch < 26; ++ch) {
                Node* y = search(x->next[ch], word, d+1);
                if (y && y->value) {
                    return y;
                }
            }
            return nullptr;
        } else {
            int ch = word[d] - 'a';
            return search(x->next[ch], word, d+1);
        }
    }

public:
    WordDictionary() {
        
    }
    
    void addWord(string word) {
        root = addWord(root, word, 0);
    }
    
    bool search(string word) {
        Node* x = search(root, word, 0);
        if (x == nullptr) return false;
        return x->value;
    }
};
