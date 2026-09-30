class LRUCache {
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
            // In std::unordered_map, calling insert() with an existing key does nothing and will not overwrite the existing value.
            // Bug: hash_map_.insert({key, container_list_.begin()});
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
        // In std::unordered_map, calling insert() with an existing key does nothing and will not overwrite the existing value.
        // Bug: hash_map_.insert({key, container_list_.begin()});
        hash_map_[key] = container_list_.begin();
    }

private:
    list<pair<int/*key*/, int/*val*/>> container_list_;
    unordered_map<int/*key*/, list<pair<int/*key*/, int/*val*/>>::iterator> hash_map_;
    size_t capacity_;
};
