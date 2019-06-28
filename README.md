
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