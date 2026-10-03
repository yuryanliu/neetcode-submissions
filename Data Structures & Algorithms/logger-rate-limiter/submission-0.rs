struct Logger {
    messages: HashSet<String>,
    queue: VecDeque<(i32 /*expire_timestamp*/, String /*message*/)>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            messages: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        self.cleanup(timestamp);
        if self.messages.contains(&message) {
            false
        } else {
            self.messages.insert(message.clone());
            self.queue.push_back((timestamp+10, message));
            true
        }
    }

    fn cleanup(&mut self, now: i32) {
        while let Some(front) = self.queue.front() && front.0 <= now {
            let (_, message) = self.queue.pop_front().unwrap();
            self.messages.remove(&message);
        }
    }
}
