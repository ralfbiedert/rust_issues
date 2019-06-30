
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

```
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

 