
# Basic Problem

```
=== Allocation Statistics ===
"()" -- Events: 0, Bytes: 0
"Box::new(1)" -- Events: 1, Bytes: 4
"pool = ThreadPoolBuilder::new().num_threads(4).build()" -- Events: 65, Bytes: 4942
"pool.install(|| {}) -- 1st time" -- Events: 23, Bytes: 5608
"pool.install(|| {}) -- 2nd time" -- Events: 2, Bytes: 24
"pool.install(|| {}) -- next 1000 times" -- Events: 2032, Bytes: 48064
"drop(pool)" -- Events: 0, Bytes: 0
```


# Source of Allocation

- `ThreadPool::install` invokes `Registry::in_worker`
- There the check `worker_thread.is_null()` constantly fails, so it invokes `self.in_worker_cold(op)`.
- That in turn does a `LockLatch::new()`, which causes the following allocations: 

```
"Mutex::new" -- Events: 1, Bytes: 16
"Condvar::new" -- Events: 1, Bytes: 8
```

- The `worker_thread.is_null` check fails because `WorkerThread::current()` always returns `0x0`.

- Which is a bit puzzling, because I can observe that `WorkerThread::set_current(worker_thread);` is invoked from `main_loop` (multiple times),
and 

```rust
println!("main_loop() - before set {:?}", WorkerThread::current());
WorkerThread::set_current(worker_thread);
println!("main_loop() - after set {:?}", WorkerThread::current());
``` 

- each prints something like: 

```
main_loop() - before set 0x0
main_loop() - after set 0x635feff240
```                                           




# Trying to Fix

- Putting LockLatch on TLS technically works, but `Condvar` doesn't reset so can't be reused.
- Have to replace condvar.
- Hacked it with `AtomicBoolean` and hot loop, seems to work.


# Other source of allocation

- There is a 2nd source of allocation
- Every once in a while (apparently every 32 invocations precisely):

```
"self.in_worker_cold(op)" -- Events: 1, Bytes: 752
"self.in_worker_cold(op)" -- Events: 0, Bytes: 0
...
... 32 invocations later ...
... 
"self.in_worker_cold(op)" -- Events: 0, Bytes: 0
"self.in_worker_cold(op)" -- Events: 1, Bytes: 752
```
 
 - Seems to come from `Registry::inject` ... `self.injected_jobs.push(job_ref)`, which is `SegQueue::push`
 

- If we look at simple test case 

```rust
let x: SegQueue<u32> = SegQueue::new();

for i in 0..100 {
    measure("push_pop", || {
        x.push(1);
        x.pop();
    });
}
```


- Despite the intuition that this should never use more than 1 "memory slot, every 32 calls:

```
"push_pop" -- Events: 1, Bytes: 504
"push_pop" -- Events: 0, Bytes: 0
...
... 32 invocations later ...
... 
"push_pop" -- Events: 0, Bytes: 0
"push_pop" -- Events: 1, Bytes: 504

```

- Allocation is caused by these lines:

```rust
if offset + 1 == BLOCK_CAP && next_block.is_none() {
    next_block = Some(Box::new(Block::<T>::new()));
}
```

- Which in turn comes from the tail just growing 

```
offset 0 ... tail 0
offset 1 ... tail 2
offset 2 ... tail 4
offset 3 ... tail 6
offset 4 ... tail 8
offset 5 ... tail 10
offset 6 ... tail 12
offset 7 ... tail 14
offset 8 ... tail 16
offset 9 ... tail 18
offset 10 ... tail 20
offset 11 ... tail 22
offset 12 ... tail 24
offset 13 ... tail 26
offset 14 ... tail 28
offset 15 ... tail 30
offset 16 ... tail 32
offset 17 ... tail 34
offset 18 ... tail 36
offset 19 ... tail 38
offset 20 ... tail 40
offset 21 ... tail 42
offset 22 ... tail 44
offset 23 ... tail 46
offset 24 ... tail 48
offset 25 ... tail 50
offset 26 ... tail 52
offset 27 ... tail 54
offset 28 ... tail 56
offset 29 ... tail 58
offset 30 ... tail 60
offset 0 ... tail 64
```