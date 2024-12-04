use day_04::{find_xmas, find_xmas_shapes};

#[test]
fn should_find_xmas() {
    assert_eq!(find_xmas("tests/resources/puzzle.txt"), 18);
}

#[test]
fn should_find_xmas_shapes() {
    assert_eq!(find_xmas_shapes("tests/resources/puzzle_xmas_shapes.txt"), 9);
}