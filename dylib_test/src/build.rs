fn main() {
    cc::Build::new()
        .files(vec!["src/would_be_generated.c"])
        .warnings(true)
        .extra_warnings(true)
        .compile("test_harness");
}