#[derive(Debug, PartialEq)]
struct S(u8);

#[test]
fn ice() {
    let s = S(0);
    assert_eq!(s, S([][0]));
}
