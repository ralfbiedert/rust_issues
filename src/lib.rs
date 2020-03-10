#[derive(Debug, PartialEq)]
struct S(u8);

#[test]
fn ice() {
    assert_eq!(S(0), S([][0]));
}
