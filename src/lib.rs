use std::fs::read_to_string;

pub fn find_xmas(input_path: &str) -> u32 {
    let input = read_input(&input_path);

    search_xmas(input)
}

pub fn find_xmas_shapes(input_path: &str) -> u32 {
    let input = read_input(&input_path);

    search_xmas_shapes(input)
}

fn search_xmas_shapes(input: Vec<Vec<char>>) -> u32 {
    if input.is_empty() {
        return 0;
    }

    let rows = input.len();
    let cols = input[0].len();
    let mut xmas_count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if r + 1 < rows && r > 0 && c + 1 < cols && c > 0 {
                if input[r][c] == 'A' {
                    let first_diagonal = vec![input[r - 1][c - 1], input[r + 1][c + 1]];
                    let second_diagonal = vec![input[r - 1][c + 1], input[r + 1][c -1]];
                    if first_diagonal.iter().filter(|value| **value == 'M').count() == 1 && first_diagonal.iter().filter(|value| **value == 'S').count() == 1 
                        && second_diagonal.iter().filter(|value| **value == 'M').count() == 1 && second_diagonal.iter().filter(|value| **value == 'S').count() == 1 {
                    xmas_count += 1;
                }
            }
        }

    }
}

    xmas_count
}

fn search_xmas(input: Vec<Vec<char>>) -> u32 {
    if input.is_empty() {
        return 0;
    }

    let rows = input.len();
    let cols = input[0].len();
    let matching_pattern = vec!['X', 'M', 'A', 'S'];
    let mut xmas_count = 0;

    let get_char = |r: isize, c: isize| -> Option<char> {
        if r >= 0 && (r as usize) < rows && c >= 0 && (c as usize) < cols {
            Some(input[r as usize][c as usize])
        } else {
            None
        }
    };

    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1)
    ];

    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] == 'X' {
                for &(row_offset, col_offset) in &directions {
                    let mut matching_index = 1;
                    let mut current_row = r as isize + row_offset;
                    let mut current_col = c as isize + col_offset;

                    while matching_index < matching_pattern.len() {
                        if let Some(next_char) = get_char(current_row, current_col) {
                            if next_char == matching_pattern[matching_index] {
                                matching_index += 1;
                                current_row += row_offset;
                                current_col += col_offset;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    if matching_index == matching_pattern.len() {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    xmas_count
}

fn read_input(input_path: &str) -> Vec<Vec<char>> {
    read_to_string(input_path)
    .unwrap()
    .lines()
    .map(|line| line.chars().collect())
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_2_xmas_shapes_reversed_and_not_reversed() {
        assert_eq!(
            search_xmas_shapes(vec![
                vec!['M', 'O', 'M', 'O', 'S'],
                vec!['O', 'A', 'O', 'A', 'O'],
                vec!['S', 'O', 'S', 'O', 'M'],
                vec!['O', 'A', 'O', 'O', 'S'],
                vec!['M', 'O', 'M', 'O', 'M']
            ]),
            2
        );
    }

    #[test]
    fn should_find_1_xmas_shape_reversed() {
        assert_eq!(
            search_xmas_shapes(vec![
                vec!['S', 'O', 'S', 'O'],
                vec!['O', 'A', 'O', 'O'],
                vec!['M', 'O', 'M', 'O'],
                vec!['O', 'O', 'O', 'S']
            ]),
            1
        );
    }

    #[test]
    fn should_find_1_xmas_shape() {
        assert_eq!(
            search_xmas_shapes(vec![
                vec!['M', 'O', 'S', 'O'],
                vec!['O', 'A', 'O', 'O'],
                vec!['M', 'O', 'S', 'O'],
                vec!['O', 'O', 'O', 'S']
            ]),
            1
        );
    }

    #[test]
    fn should_find_0_xmas_shapes_when_empty() {
        assert_eq!(search_xmas_shapes(Vec::new()), 0);
    }

    #[test]
    fn should_find_1_xmas_diagonal() {
        assert_eq!(search_xmas(vec![
            vec!['X', 'O', 'O', 'O'],
            vec!['O', 'M', 'O', 'O'],
            vec!['O', 'O', 'A', 'O'],
            vec!['O', 'O', 'O', 'S']
        ]), 1);
    }

    #[test]
    fn should_find_2_xmas_verticaly() {
        assert_eq!(search_xmas(vec![
            vec!['X', 'S'],
            vec!['M', 'A'],
            vec!['A', 'M'],
            vec!['S', 'X']
        ]), 2);
    }

    #[test]
    fn should_find_1_xmas_verticaly() {
        assert_eq!(search_xmas(vec![
            vec!['X'],
            vec!['M'],
            vec!['A'],
            vec!['S']
        ]), 1);
    }

    #[test]
    fn should_find_2_xmas_when_on_same_line_containing_other_chars() {
        assert_eq!(search_xmas(vec![
            vec!['X', 'X', 'M', 'A', 'S', 'X', 'M', 'A', 'S']
        ]), 2);
    }

    #[test]
    fn should_find_1_xmas_when_on_same_line_containing_other_chars() {
        assert_eq!(search_xmas(vec![
            vec!['X', 'X', 'M', 'A', 'S', 'X']
        ]), 1);
    }

    #[test]
    fn should_find_1_xmas_when_on_same_line_reversed() {
        assert_eq!(search_xmas(vec![
            vec!['S', 'A', 'M', 'X']
        ]), 1);
    }

    #[test]
    fn should_find_1_xmas_when_on_same_line() {
        assert_eq!(search_xmas(vec![
            vec!['X', 'M', 'A', 'S']
        ]), 1);
    }

    #[test]
    fn should_find_0_xmas_when_empty() {
        assert_eq!(search_xmas(Vec::new()), 0);
    }

    #[test]
    fn should_read_file() {
        assert_eq!(read_input("tests/resources/light_puzzle.txt"), vec![
            vec!['M', 'M', 'M', 'S'],
            vec!['M', 'S', 'A', 'M'],
        ]);
    }

}