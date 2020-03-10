
```
> cargo test

   Compiling rust_issues v0.1.0 (D:\Development\Source\rust_issues)
error: index out of bounds: the len is 0 but the index is 0
 --> src\lib.rs:7:21
  |
7 |     assert_eq!(s, S([][0]));
  |                     ^^^^^
  |
  = note: `#[deny(const_err)]` on by default

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', src\librustc\mir\interpret\value.rs:305:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (6d77e45f0 2019-12-04) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

error: could not compile `rust_issues`.

To learn more, run the command again with --verbose.
```
