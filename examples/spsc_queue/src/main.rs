use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering::*};
use std::sync::Arc;
use std::thread;


// Lock-free single-producer / single-consumer queue over a fixed arena.
struct Queue {
    arena: Vec<AtomicU64>,

    head: AtomicUsize, // only the consumer moves this
    tail: AtomicUsize, // only the producer moves this
}


impl Queue {
    fn new(cap: usize) -> Self {
        let arena = (0..cap).map(|_| AtomicU64::new(0)).collect();

        Queue {
            arena,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }


    fn push(&self, v: u64) -> bool {
        let tail = self.tail.load(Relaxed);
        let head = self.head.load(Acquire);

        if tail.wrapping_sub(head) == self.arena.len() {
            return false; // full
        }

        self.arena[tail % self.arena.len()].store(v, Relaxed);

        self.tail.store(tail.wrapping_add(1), Release); // publish

        true
    }


    fn pop(&self) -> Option<u64> {
        let head = self.head.load(Relaxed);
        let tail = self.tail.load(Acquire);

        if head == tail {
            return None; // empty
        }

        let v = self.arena[head % self.arena.len()].load(Relaxed);

        self.head.store(head.wrapping_add(1), Release); // free the slot

        Some(v)
    }


    fn push_spin(&self, v: u64) {
        while !self.push(v) {}
    }


    fn pop_spin(&self) -> u64 {
        loop {
            if let Some(v) = self.pop() {
                return v;
            }
        }
    }
}


fn main() {
    let q = Arc::new(Queue::new(16));


    // producer thread
    let producer = {
        let q = q.clone();

        thread::spawn(move || {
            (1..=100).for_each(|i| q.push_spin(i))
        })
    };


    // consumer (main thread)
    let sum: u64 = (0..100).map(|_| q.pop_spin()).sum();

    producer.join().unwrap();


    println!("sum = {sum}"); // expect 5050
}
