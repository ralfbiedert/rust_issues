#[link(name = "test_harness")]
extern "C" {
    fn test_harness() -> i32;
}

// Issue 01 - Each Mac and Linux accept `dylib`, but Windows needs an extra `dylib.dll`, because
// Rust generates a `dylib.dll.lib` ... It would be nice if it just resolved #[link(name = "dylib")]
// as well.
// #[link(name = "dylib")]   // Mac & Linux
#[link(name = "dylib.dll")] // Windows ...
extern "C" {}

#[test]
fn c_api_tests_work() {
    let rval = unsafe { test_harness() };
    
    assert_eq!(rval, 0);
}