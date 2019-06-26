
#[cfg(not(feature="test"))]
fn f() {
    compile_error!("XX");
}

fn main() {

}