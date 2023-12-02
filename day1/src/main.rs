use std::sync::OnceLock;

// Why not an actual HashMap? I plan on iterating over EVERY entry to naively check each line,
// so we don't need to add the hashing overhead on insert.
fn get_number_map() -> &'static Vec<(&'static str, u32)> {
    static NUMBER_MAP: OnceLock<Vec<(&'static str, u32)>> = OnceLock::new();
    NUMBER_MAP.get_or_init(|| {
        vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]
    })
}

#[derive(Debug, PartialEq, Eq)]
struct LineMatch {
    first: u32,
    last: u32,
}

fn get_line_match(s: &str) -> LineMatch {
    // Get all matches, but take the one whose index is closest to the start of the string
    let mut first_matches: Vec<_> = get_number_map()
        .iter()
        .filter_map(|(pattern, digit)| {
            s.find(pattern).map(|idx| (idx, *digit))
        })
        .collect();
    first_matches.sort();

    // Get all matches again, but this time use `rfind` and take the index closest to the _end_ of the string
    let mut last_matches: Vec<_> = get_number_map()
        .iter()
        .filter_map(|(pattern, digit)| {
            s.rfind(pattern).map(|idx| (idx, *digit))
        })
        .collect();
    last_matches.sort();

    if first_matches.is_empty() || last_matches.is_empty() {
        LineMatch {first: 0, last: 0}
    } else {
        let first = first_matches.first().expect("first_matches should contain something").1;
        let last = last_matches.last().expect("last_matches should contain something").1;
        LineMatch {first, last}
    }
}

fn main() {
    let input = include_str!("../data/input.txt").trim_end();

    let sum: u32 = input
        .lines()
        .map(|line| {
            let line_match = get_line_match(line);
            (line_match.first * 10) + line_match.last
        })
        .sum();

    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use crate::get_line_match;

    #[test]
    fn test_get_line_match() {
        for (s, first, last) in vec![
            // Part 1 examples
            ("1abc2", 1u32, 2u32),
            ("pqr3stu8vwx", 3, 8),
            ("a1b2c3d4e5f", 1, 5),
            ("treb7uchet", 7, 7),
            // Part 2 examples
            ("two1nine", 2, 9),
            ("eightwothree", 8, 3),
            ("abcone2threexyz", 1, 3),
            ("xtwone3four", 2, 4),
            ("4nineeightseven2", 4, 2),
            ("zoneight234", 1, 4),
            ("7pqrstsixteen", 7, 6),
            // Default case
            ("goobus", 0, 0),
            ("one", 1, 1),
            ("1eightwo", 1, 2),
        ] {
            let m = get_line_match(s);
            assert_eq!(first, m.first);
            assert_eq!(last, m.last);
        }
    }
}