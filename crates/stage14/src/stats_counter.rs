use std::sync::Arc;
use crossbeam_utils::CachePadded;
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

// 🚀 The Optimization Stage: Resolving False Sharing.
// ❓ We've identified the "Invalidation Storm" caused by independent counters
//   sharing a single 64-byte cache line.
// 🤔 Questions:
// - How does isolating each AtomicUsize on its own cache line stop "Ping-Ponging"?
// - Why does this result in a significant speedup on modern multi-core processors?
// - What is the MESI protocol (Modified, Exclusive, Shared, Invalid)?
// - How does updating one counter force other CPU cores into the "Invalid" state
//   for the ENTIRE 64-byte block?
#[repr(transparent)]
struct Counter(CachePadded<AtomicUsize>);

pub(crate) struct PaddedStatsCounter {
    entries: [Counter; 4],
}

impl PaddedStatsCounter {
    // ❓ Initializing an array of Non-Copy types is a classic Rust puzzle.
    // 🤔 Questions: 
    // - AtomicUsize and CachePadded do not implement the 'Copy' trait.
    // - Why would [Counter(...); 4] fail to compile in this case?
    // - What is the 'const { ... }' block doing for each array element?
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            entries: [const { Counter(CachePadded::new(AtomicUsize::new(0))) }; 4],
        })
    }

    pub(crate) fn increase_hits(&self) {
        self.increase(StatsType::HITS, 1);
    }

    pub(crate) fn increase_misses(&self) {
        self.increase(StatsType::MISSES, 1);
    }

    pub(crate) fn increase_gets(&self) {
        self.increase(StatsType::GETS, 1);
    }

    pub(crate) fn increase_puts(&self) {
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

    fn hit_ratio(&self) -> f64 {
        let (hits, misses) = (self.hits(), self.misses());
        if misses == 0 {
            return 0.0;
        }
        // 💡 Hit Ratio = hits / (hits + misses)
        hits as f64 / (hits + misses) as f64
    }

    fn increase(&self, stats_type: StatsType, count: usize) {
        // ❓ We use 'AcqRel' (Acquire-Release) to push this update to other cores.
        // 🤔 Question: Why is this sync point necessary for a consistent 'hit_ratio' read?
        self.entries[stats_type as usize].0.fetch_add(count, Ordering::AcqRel);
    }

    fn get(&self, stats_type: StatsType) -> usize {
        // ❓ We use 'Acquire' to pull latest updates from shared caches.
        // 🤔 Question: What happens if we use 'Ordering::Relaxed' here instead?
        // (Hint: Think about "staleness" vs "correctness").
        self.entries[stats_type as usize].0.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_hits_is_zero() {
        let stats_counter = PaddedStatsCounter::new();
        assert_eq!(stats_counter.hits(), 0);
    }

    #[test]
    fn initial_misses_is_zero() {
        let stats_counter = PaddedStatsCounter::new();
        assert_eq!(stats_counter.misses(), 0);
    }

    #[test]
    fn initial_hit_ratio_is_zero() {
        let stats_counter = PaddedStatsCounter::new();
        assert_eq!(stats_counter.hit_ratio(), 0.0);
    }

    #[test]
    fn increase_hits() {
        let stats_counter = PaddedStatsCounter::new();
        stats_counter.increase_hits();
        assert_eq!(stats_counter.hits(), 1);
    }

    #[test]
    fn increase_misses() {
        let stats_counter = PaddedStatsCounter::new();
        stats_counter.increase_misses();
        assert_eq!(stats_counter.misses(), 1);
    }

    #[test]
    fn increase_gets() {
        let stats_counter = PaddedStatsCounter::new();
        stats_counter.increase_gets();
        assert_eq!(stats_counter.get(StatsType::GETS), 1);
    }

    #[test]
    fn increase_puts() {
        let stats_counter = PaddedStatsCounter::new();
        stats_counter.increase_puts();
        assert_eq!(stats_counter.get(StatsType::PUTS), 1);
    }

    #[test]
    fn hit_ratio() {
        let stats_counter = PaddedStatsCounter::new();
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
        let shared_stats = Arc::new(PaddedStatsCounter::new());
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

        // 10 threads * 1000 increments = 10,000 hits total
        assert_eq!(shared_stats.hits(), 10000);
    }

    #[test]
    fn concurrent_miss_increments() {
        let shared_stats = Arc::new(PaddedStatsCounter::new());
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

        // 10 threads * 1000 misses = 10,000 misses total
        assert_eq!(shared_stats.misses(), 10000);
    }

    #[test]
    fn concurrent_put_increments() {
        let shared_stats = Arc::new(PaddedStatsCounter::new());
        let mut thread_handles = vec![];

        for _ in 0..10 {
            let stats_counter = Arc::clone(&shared_stats);
            thread_handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    stats_counter.increase_puts();
                }
            }));
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        assert_eq!(shared_stats.puts(), 10000);
    }

    #[test]
    fn concurrent_get_increments() {
        let shared_stats = Arc::new(PaddedStatsCounter::new());
        let mut thread_handles = vec![];

        for _ in 0..10 {
            let stats_counter = Arc::clone(&shared_stats);
            thread_handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    stats_counter.increase_gets();
                }
            }));
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        assert_eq!(shared_stats.gets(), 10000);
    }
}
