struct Node {
    bool val;
    Node* next[26];
    Node() {
        val = false;
        for (int c = 0; c < 26; ++c) {
            next[c] = nullptr;
        }
    }
};

class WordDictionary {
private:
    Node* root = nullptr;

    Node* addWord(Node* x, string& word, int d) {
        if (!x) x = new Node();
        if (d == word.size()) { x->val = true; return x; }        
        int c = word[d] - 'a';
        x->next[c] = addWord(x->next[c], word, d+1);
        return x;
    }
    
    Node* search(Node* x, string& word, int d) {
        if (!x) return nullptr;
        if (d == word.size()) return x;
        if (word[d] == '.') {
            for (int c = 0; c < 26; ++c) {
                Node* find = search(x->next[c], word, d+1);
                if (find) {
                    return find;
                }
            }
            return nullptr;
        } else {
            int c = word[d] - 'a';        
            return search(x->next[c], word, d+1);
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
        if (!x) return false;
        return x->val;
    }
};
