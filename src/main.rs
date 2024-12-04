use day_04::{find_xmas, find_xmas_shapes};

fn main() {

    let xmas_in_word_search = find_xmas("src/resources/puzzle.txt");

    println!("Xmas occurencies in puzzle: {xmas_in_word_search}");

    let xmas_in_word_search = find_xmas_shapes("src/resources/puzzle.txt");

    println!("Xmas shapes occurencies in puzzle: {xmas_in_word_search}");
}
