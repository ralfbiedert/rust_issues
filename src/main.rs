#![feature(allocator_api)]

use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::alloc::{System, Alloc, Layout, GlobalAlloc};
use std::ptr::NonNull;
use std::thread::ThreadId;
use std::thread;
use rayon::ThreadPoolBuilder;


pub struct UserAlloc;

#[global_allocator]
static mut ALLOCATOR: UserAlloc = UserAlloc;



#[derive(Debug, Copy, Clone)]
struct Event {
    threadid: Option<ThreadId>,
    time: usize,
    size: usize,
    align: usize,
}

impl Event {
    const fn new() -> Self {
        Self {
            threadid: None,
            time: 0,
            size: 0,
            align: 0
        }
    }
}


/// All events we keep track of
static mut EVENTS: [Event; 1000000]  = [Event::new(); 1000000];

/// Global pointer to next event to write
static mut NEXT: AtomicUsize = AtomicUsize::new(0);

/// If set to true, our allocator will record.
static mut TRACK: AtomicBool = AtomicBool::new(false);


unsafe impl GlobalAlloc for UserAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if TRACK.load(Ordering::SeqCst) {
            // let x = thread::current(); // crash ?!
            EVENTS[NEXT.load(Ordering::SeqCst)] = Event {
                threadid: None,
                size: layout.size(),
                align: layout.align(),
                time: 0
            };
        
            NEXT.fetch_add(1, Ordering::SeqCst);
        }
    
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}



fn measure<T: FnOnce() -> R, R>(label: &str, x: T) -> R {
    unsafe {
        TRACK.store(true, Ordering::SeqCst);
        
        let r = x();
        
        TRACK.store(false, Ordering::SeqCst);
        
        let N = NEXT.load(Ordering::SeqCst);
        let mut total_bytes = 0;
        
        for i in 0 .. N {
            let event = EVENTS[i];
            
            total_bytes += event.size;
        }
        
        println!("{:?} -- Events: {:?}, Bytes: {:?}", label, N, total_bytes);
        
        NEXT.store(0, Ordering::SeqCst);
        
        r
    }
}

pub fn main() {
    // DON'T TRY THIS AT HOME, KIDS
    unsafe {
        println!("=== Allocation Statistics ===");
        
        measure("()", || {
            ()
        });
        
        measure("Box::new(1)", || {
            Box::new(1);
        });
        
        let mut pool = measure("pool = ThreadPoolBuilder::new().num_threads(4).build()", || {
            ThreadPoolBuilder::new().num_threads(4).build().unwrap()
        });
    
        measure("pool.install(|| {}) -- 1st time", || {
            pool.install(|| {
            
            })
        });
    
        measure("pool.install(|| {}) -- 2nd time", || {
            pool.install(|| {
            
            })
        });
    
        measure("pool.install(|| {}) -- next 1000 times", || {
            for i in 0 .. 1000 {
                pool.install(|| {
        
                })
            }
        });
    
        measure("drop(pool)", || {
            drop(pool)
        });
    }
    
}