use achtung_allocation::measure;
use crossbeam_queue::SegQueue;
use rayon::ThreadPoolBuilder;
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    // DON'T TRY THIS AT HOME, KIDS
    unsafe {
        println!("=== Allocation Statistics ===");

        let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

        pool.install(|| {});
        pool.install(|| {});

        for i in 0..100 {
            pool.install(|| {})
        }

        for i in 0..100 {
            pool.install(|| {})
        }

        let x: SegQueue<u32> = SegQueue::new();

        for i in 0..100 {
            //            measure("push_pop", || {
            x.push(1);
            x.pop();
            //            });
        }
    }
}
