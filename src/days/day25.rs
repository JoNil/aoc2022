pub static INPUT: &str = include_str!("../input/25.txt");
pub static TEST_INPUT: &str = include_str!("../input/25_test.txt");

pub fn a(input: &str) -> String {
    String::new()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), "2=-1=0".to_string());
    assert_eq!(a(INPUT), String::new());
}

pub fn b(input: &str) -> String {
    String::new()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), String::new());
    assert_eq!(b(INPUT), String::new());
}
