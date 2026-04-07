use std::sync::atomic::{AtomicUsize, Ordering};

// ❓ Recording metrics in a high-concurrency environment is a unique challenge.
// 🤔 Questions: 
// - Why is using a 'Mutex' or 'RwLock' around a simple integer counter "overkill"?
// - What is the "Lock Tax"? (Hint: Think about OS context switches vs hardware instructions).
enum StatsType {
    HITS = 0,
    MISSES = 1,
    GETS = 2,
    PUTS = 3,
}

// 💡 The Hardware's Uninterruptible Promise.
// ❓ Atomics ensure that operations are "indivisible" at the CPU level.
// 🤔 Questions: 
// - Why can we mutate the 'entries' array through a shared reference (&self)?
// - What is "Internal Mutability" (or Shared Mutability) in the context of Atomics?
struct ConcurrentStatsCounter {
    entries: [AtomicUsize; 4],
}

impl ConcurrentStatsCounter {
    fn new() -> Self {
        Self {
            entries: [
                AtomicUsize::new(0),
                AtomicUsize::new(0),
                AtomicUsize::new(0),
                AtomicUsize::new(0),
            ],
        }
    }

    fn increase_hits(&self) {
        self.increase(StatsType::HITS, 1);
    }

    fn increase_misses(&self) {
        self.increase(StatsType::MISSES, 1);
    }

    fn increase_gets(&self) {
        self.increase(StatsType::GETS, 1);
    }

    fn increase_puts(&self) {
        self.increase(StatsType::PUTS, 1);
    }

    fn hits(&self) -> usize {
        self.get(StatsType::HITS)
    }

    fn misses(&self) -> usize {
        self.get(StatsType::MISSES)
    }

    fn puts(&self) -> usize {
        self.get(StatsType::PUTS)
    }

    fn gets(&self) -> usize {
        self.get(StatsType::GETS)
    }

    // 🚀 The Goal: Calculating the Hit Ratio.
    // 🤔 Questions:
    // - Why do we check 'misses == 0'? (The "Infinity" problem).
    fn hit_ratio(&self) -> f64 {
        let (hits, misses) = (self.hits(), self.misses());
        if misses == 0 {
            return 0.0;
        }
        hits as f64 / (hits + misses) as f64
    }

    // ❓ Choosing the right Memory Ordering.
    // 🤔 Questions: 
    // - We use 'AcqRel' (Acquire-Release) for updates and 'Acquire' for reads.
    // - How does this build a "Happens-Before" relationship between threads?
    // - In a simple counter, why might 'Ordering::Relaxed' be even faster?
    fn increase(&self, stats_type: StatsType, count: usize) {
        // 💡 Shared Mutability: Mutating state through &self.
        unimplemented!()
    }

    // ❓ The Invisible Storm (The Final Boss of Concurrency).
    // 💡 Performance Note:
    // CPUs own data in 64-byte chunks called "Cache Lines".
    // Because our counters share a single array, they likely sit on the SAME line.
    // 🤔 Question: What is "False Sharing"? 
    // - Why does updating 'HITS' on Core 1 force Core 2 to trash its cache of 'MISSES'?
    // - How does this "Cache Line Ping-Pong" destroy scaling performance?
    fn get(&self, stats_type: StatsType) -> usize {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_hits_is_zero() {
        let stats_counter = ConcurrentStatsCounter::new();
        assert_eq!(stats_counter.hits(), 0);
    }

    #[test]
    fn initial_misses_is_zero() {
        let stats_counter = ConcurrentStatsCounter::new();
        assert_eq!(stats_counter.misses(), 0);
    }

    #[test]
    fn initial_hit_ratio_is_zero() {
        let stats_counter = ConcurrentStatsCounter::new();
        assert_eq!(stats_counter.hit_ratio(), 0.0);
    }

    #[test]
    fn increase_hits() {
        let stats_counter = ConcurrentStatsCounter::new();
        stats_counter.increase_hits();
        assert_eq!(stats_counter.hits(), 1);
    }

    #[test]
    fn increase_misses() {
        let stats_counter = ConcurrentStatsCounter::new();
        stats_counter.increase_misses();
        assert_eq!(stats_counter.misses(), 1);
    }

    #[test]
    fn increase_gets() {
        let stats_counter = ConcurrentStatsCounter::new();
        stats_counter.increase_gets();
        assert_eq!(stats_counter.get(StatsType::GETS), 1);
    }

    #[test]
    fn increase_puts() {
        let stats_counter = ConcurrentStatsCounter::new();
        stats_counter.increase_puts();
        assert_eq!(stats_counter.get(StatsType::PUTS), 1);
    }

    #[test]
    fn hit_ratio() {
        let stats_counter = ConcurrentStatsCounter::new();
        stats_counter.increase_hits();
        stats_counter.increase_hits();
        stats_counter.increase_misses();
        stats_counter.increase_misses();

        assert_eq!(stats_counter.hit_ratio(), 0.5);
    }
}

#[cfg(test)]
mod concurrency_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn concurrent_hits_increment() {
        let shared_stats = Arc::new(ConcurrentStatsCounter::new());
        let mut thread_handles = vec![];

        for _ in 0..10 {
            let stats_counter = Arc::clone(&shared_stats);
            thread_handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    stats_counter.increase_hits();
                }
            }));
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        assert_eq!(shared_stats.hits(), 10000);
    }

    #[test]
    fn concurrent_miss_increments() {
        let shared_stats = Arc::new(ConcurrentStatsCounter::new());
        let mut thread_handles = vec![];

        for _ in 0..10 {
            let stats_counter = Arc::clone(&shared_stats);
            thread_handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    stats_counter.increase_misses();
                }
            }));
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        assert_eq!(shared_stats.misses(), 10000);
    }
}
