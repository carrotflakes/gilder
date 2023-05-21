fn main() {}

#[test]
fn test() {
    use gilder::assert_golden;

    assert_golden!(123);
    assert_golden!("hello");
    assert_golden!("a\nb\nc");
}
