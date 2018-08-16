use serde_derive::Serialize;

#[derive(Serialize)]
enum E {
    // `crate` seems to be causing this
    X,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(C)]
pub struct Matrix4f32 {
    crate data: [f32; 16],
}

fn main() {}
