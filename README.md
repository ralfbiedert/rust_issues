
# Background

We have a Rust project `dylib` where we want to produce a shared library (`.dll`, `.dylib`, `.so`) for various 
platforms. 

As part of the build process we also want to verify the provided headers match the library. 

Do do that we have a separate project `dylib_test`, where we extract C doc tests into individual 
`generated_nnn.c` files, produce a `generated.rs` file that knows all C tests, and emits a 
`#[test]` for each, which we then want to run with `cargo test`.

The `dylib` project should only emit a `cdylib`, and the `dylib_test` should only link against that
`cdylib`, to make the actual `.dll`, ... works.  

   

# Problems

### Linking dylib.dll

When linking against `dylib`, the `#[link(name = "dylib")]` attribute behaves inconsistently. On 
Mac and Linux the attribute works. On Windows, it fails with an 

```
error: linking with `link.exe` failed: exit code: 1181
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\VC\\Tools\\MSVC\\14.21.27702\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.17sc3p6xub1nskk5.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.1d4yl2ra8f02uq65.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.2boci68ky9pbilmt.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.2kg3da7ad2y6u5t4.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.328elmwbbr69laus.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.3pjpoaqleh1qku54.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.48cxjx0647kyqroa.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.4xn06m0s17xpy7q7.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.bdw34kj2m6ietun.rcgu.o" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.dxf6w5mo3hfjrm.rcgu.o" "/OUT:D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.exe" "D:\\Development\\Source\\rust_issues\\target\\debug\\deps\\dylib_test-16b85ad62e2643c4.3tou2046e23p3ckg.rcgu.o" "/OPT:REF,NOICF" "/DEBUG" "/NATVIS:C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libcore.natvis" "/LIBPATH:D:\\Development\\Source\\rust_issues\\target\\debug\\deps" "/LIBPATH:D:\\Development\\Source\\rust_issues\\target\\debug\\build\\dylib_test-df967a17b396e952\\out" "/LIBPATH:C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "dylib.lib" "test_harness.lib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libtest-5868cd1efdb1a14a.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libterm-4a1cd4f9337faede.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libgetopts-c94244661cc3c49c.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libunicode_width-5d83c260a3fd1c81.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd-69ffdb8026c42481.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libpanic_unwind-324700ef1bcafc8b.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libbacktrace-66fccb33cfc77b7d.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_demangle-429461051376b1a5.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libhashbrown-16c7dc58d869d64a.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_alloc-b5f6113de773ee4b.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libunwind-52ee0b65e04bcb17.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcfg_if-eee1f1d0cc78c8f9.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liblibc-f5bd27a471d11e8c.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc-bb531cfdfdeadec5.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_core-48d94702399abeaa.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-edaeab1200806e65.rlib" "C:\\Users\\rb\\.rustup\\toolchains\\nightly-2019-07-08-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-5cb137db32dc726e.rlib" "kernel32.lib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "msvcrt.lib"
  = note: LINK : fatal error LNK1181: cannot open input file 'dylib.lib'
```  

I believe this is because Rust on Windows produces a `dylib.dll` and `dylib.dll.lib` (instead of a `dylib.lib`). When then resolving 
the library someone Rust apparently isn't aware that to resolve `name="dylib"` it should not only look for `dylib.lib`, 
but instead also for `dylib.dll.lib`.




### Running cargo test --release

When linking against `dylib`, the `#[link(name = "dylib")]` on Linux, Rust seems to place the 
linker arguments in the wrong order. If I execute `cargo test --release` on a regular Ubuntu 18.04, I get 
the following error message:


```
   Compiling dylib_test v0.1.0 (/home/rb/source/rust_issues/dylib_test)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/deps/dylib_test-38f2a187e2076f50.dylib_test.zejn0h82-cgu.0.rcgu.o" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/deps/dylib_test-38f2a187e2076f50.dylib_test.zejn0h82-cgu.1.rcgu.o" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/deps/dylib_test-38f2a187e2076f50.dylib_test.zejn0h82-cgu.2.rcgu.o" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/deps/dylib_test-38f2a187e2076f50.dylib_test.zejn0h82-cgu.3.rcgu.o" "-o" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/deps/dylib_test-38f2a187e2076f50" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/deps/dylib_test-38f2a187e2076f50.17cl81ouu8r6hb2h.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/deps" "-L" "/home/rb/source/rust_issues/target/release/deps" "-L" "/home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/build/dylib_test-79d0299f44818f6a/out" "-L" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-ldylib" "-Wl,-Bstatic" "-Wl,--whole-archive" "-ltest_harness" "-Wl,--no-whole-archive" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-e0801b53662ffc3d.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-cdfff332358115a2.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-62071f29a3dbf8f6.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-4b58e49d8af86ebc.rlib" "-Wl,--start-group" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-01f0f08cf2f2f6b6.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-9f0c2fa34243df71.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-81b8dc5cee87c54f.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-5ce6b7614ea27cf6.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-d24f36a9b5d22481.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-012c35d529026afb.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-399b288ebcb05138.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-6f4dc2e9bc06e747.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-ed847dc2c2fa0fa7.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-391fc13a5e3777b7.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-7ad47fc2d32a5f8d.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-fc50d0a6a1e38bf8.rlib" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-8bfbea44a5cccf7b.rlib" "-Wl,--end-group" "/home/rb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-af944f07dc08b653.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
  = note: /home/rb/source/rust_issues/target/x86_64-unknown-linux-gnu/release/build/dylib_test-79d0299f44818f6a/out/libtest_harness.a(would_be_generated.o): In function `test_harness':
          would_be_generated.c:(.text.test_harness+0x3): undefined reference to `get_version'
          collect2: error: ld returned 1 exit status

```

Interestingly `-ldylib` (which contains `get_version`) is already in that command.  When I run the same `cc` with an extra `-ldylib` added to the end works. This is related to [this Stackoverflow](https://stackoverflow.com/questions/10456581/undefined-reference-to-symbol-even-when-nm-indicates-that-this-symbol-is-present/) comment, that apparently

> Libraries must be listed after the objects that use them (more precisely, a library will be used only if it contains a symbol that satisfies an undefined reference known at the time it is encountered). 


 