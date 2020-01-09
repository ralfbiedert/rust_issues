Running `cbindgen -c cbindgen.toml` produces:

```
WARN: Parsing crate `serde_derive`: can't find lib.rs with `cargo metadata`.
thread 'main' panicked at 'IntoIter is not generic', /Users/rb/.cargo/registry/src/github.com-1ecc6299db9ec823/cbindgen-0.12.1/src/bindgen/ir/opaque.rs:107:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

Backtrace:

```
WARN: Parsing crate `serde_derive`: can't find lib.rs with `cargo metadata`.
thread 'main' panicked at 'IntoIter is not generic', /Users/rb/.cargo/registry/src/github.com-1ecc6299db9ec823/cbindgen-0.12.1/src/bindgen/ir/opaque.rs:107:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::continue_panic_fmt
   7: std::panicking::begin_panic_fmt
   8: <cbindgen::bindgen::ir::opaque::OpaqueItem as cbindgen::bindgen::ir::item::Item>::instantiate_monomorph
   9: cbindgen::bindgen::ir::ty::Type::add_monomorphs
  10: cbindgen::bindgen::library::Library::generate
  11: cbindgen::bindgen::builder::Builder::generate
  12: cbindgen::main
  13: std::rt::lang_start::{{closure}}
  14: std::panicking::try::do_call
  15: __rust_maybe_catch_panic
  16: std::rt::lang_start_internal
  17: main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```