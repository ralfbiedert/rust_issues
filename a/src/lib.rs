#[no_mangle]
pub unsafe extern "C" fn f(x: i32) -> i32 {
    x * x
}
