class LRUCache {
private:
    size_t capacity_;
    unordered_map<int, list<pair<int, int>>::iterator> hash_map_;
    list<pair<int, int>> container_list_;
public:
    LRUCache(int capacity) {
        capacity_ = capacity;
    }
    
    int get(int key) {
        auto hash_it = hash_map_.find(key);
        if (hash_it != hash_map_.end()) {
            auto list_it = hash_it->second;
            auto key = list_it->first;
            auto val = list_it->second;
            container_list_.erase(list_it);
            container_list_.push_front({key, val});
            hash_map_[key] = container_list_.begin();
            return val;
        } else {
            return -1;
        }
    }
    
    void put(int key, int value) {
        auto hash_it = hash_map_.find(key);
        if (hash_it != hash_map_.end()) {
            auto list_it = hash_it->second;
            container_list_.erase(list_it);
        } else if (container_list_.size() == capacity_) {
            hash_map_.erase(container_list_.back().first);
            container_list_.pop_back();
        }
        container_list_.push_front({key, value});
        hash_map_[key] = container_list_.begin();
    }
};
