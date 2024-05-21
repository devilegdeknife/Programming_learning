use algorithms::collatz_length::collatz_length;

#[test]
fn collatz_length_test() {
    assert_eq!(collatz_length::collatz_length(999), 50);
}