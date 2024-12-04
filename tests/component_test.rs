use day_04::find_xmas;

#[test]
fn should_find_xmas() {
    assert_eq!(find_xmas("tests/resources/puzzle.txt"), 18);
}