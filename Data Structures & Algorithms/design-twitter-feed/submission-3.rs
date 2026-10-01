use std::collections::{HashMap, HashSet};

struct Twitter {
    timestamp: u64,
    follows: HashMap<i32, HashSet<i32>>, // self follows self too
    tweets: HashMap<i32, Vec<(u64,i32)>>,
}

impl Twitter {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            follows: HashMap::new(),
            tweets: HashMap::new(),
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let timestamp = self.timestamp; self.timestamp += 1;
        self.follows.entry(user_id).or_default().insert(user_id);
        self.tweets.entry(user_id).or_default().push((timestamp, tweet_id));
    }

    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut min_heap = BinaryHeap::<Reverse<(u64, i32)>>::new();
        if let Some(followee_ids) = self.follows.get(&user_id) {
            for followee_id in followee_ids {
                if let Some(tweets) = self.tweets.get(followee_id) {
                    for &tweet in tweets {
                        if min_heap.len() < 10 {
                            min_heap.push(Reverse(tweet));
                        } else if let Some(top) = min_heap.peek() && top.0.0 < tweet.0 {
                            min_heap.pop();
                            min_heap.push(Reverse(tweet));
                        }
                    }
                }                
            }
        }
        let mut res = vec![];
        while let Some(top) = min_heap.pop() {
            res.push(top.0.1);
        }
        res.into_iter().rev().collect()
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_default().insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_default().remove(&followee_id);
    }
}
