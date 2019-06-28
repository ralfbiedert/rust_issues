#[link(name = "a_library", kind = "dylib")]
extern "C" {
    fn f(x: i32) -> i32;
}



fn main() {
    unsafe { f(2) };
}