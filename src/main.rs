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

        //        println!("Waiting 1 second");
        sleep(Duration::new(1, 0));

        for i in 0..100 {
            pool.install(|| {})
        }

        //        println!("Waiting 1 second");
        sleep(Duration::new(1, 0));

        for i in 0..100 {
            pool.install(|| {})
        }
    }
}
