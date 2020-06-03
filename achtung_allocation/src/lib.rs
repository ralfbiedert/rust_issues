#![feature(allocator_api)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::ThreadId;

static mut EVENTS: [Event; 1000000] = [Event::new(); 1000000];
static mut NEXT: AtomicUsize = AtomicUsize::new(0);
static mut TRACK: AtomicBool = AtomicBool::new(false);

pub struct UserAlloc;

#[global_allocator]
static mut ALLOCATOR: UserAlloc = UserAlloc;

#[derive(Debug, Copy, Clone)]
struct Event {
    thread_id: Option<ThreadId>,
    time: usize,
    size: usize,
    align: usize,
}

impl Event {
    const fn new() -> Self {
        Self {
            thread_id: None,
            time: 0,
            size: 0,
            align: 0,
        }
    }
}

unsafe impl GlobalAlloc for UserAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if TRACK.load(Ordering::SeqCst) {
            // let x = thread::current(); // crash ?!
            EVENTS[NEXT.load(Ordering::SeqCst)] = Event {
                thread_id: None,
                size: layout.size(),
                align: layout.align(),
                time: 0,
            };
            
            NEXT.fetch_add(1, Ordering::SeqCst);
        }
        
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

pub fn measure<T: FnOnce() -> R, R>(label: &str, x: T) -> R {
    unsafe {
        TRACK.store(true, Ordering::SeqCst);
        
        let r = x();
        
        TRACK.store(false, Ordering::SeqCst);
        
        let N = NEXT.load(Ordering::SeqCst);
        let mut total_bytes = 0;
        
        for i in 0..N {
            let event = EVENTS[i];
            
            total_bytes += event.size;
        }
        
        println!("{:?} -- Events: {:?}, Bytes: {:?}", label, N, total_bytes);
        
        NEXT.store(0, Ordering::SeqCst);
        
        r
    }
}
