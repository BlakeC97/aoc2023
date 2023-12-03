use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::{char, digit1, multispace0, space1};
use nom::combinator::map;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};

#[derive(Debug)]
enum Color {
    Red(u64),
    Green(u64),
    Blue(u64),
}

#[derive(Debug)]
struct Game {
    id: u64,
    sets: Vec<Vec<Color>>,
}

const MAX_RED_CUBES: u64 = 12;
const MAX_GREEN_CUBES: u64 = 13;
const MAX_BLUE_CUBES: u64 = 14;

fn parse_line(input: &str) -> IResult<&str, Game> {
    // Example:
    //   Game 100: 5 blue, 5 green; 7 blue, 15 green; 4 red, 7 green, 12 blue; 7 green, 1 blue; 5 blue, 9 green, 1 red
    // Part 1: Parse the Game ID (and get rid of the colon)
    let (input, _) = take_till(|c: char| c.is_ascii_digit())(input)?;
    let (input, id) = map(digit1, |s: &str| s.parse::<u64>().expect("Cannot parse game ID as u64"))(input)?;
    let (input, _) = char(':')(input)?;

    // Part 2: Handle each individual set (digits + color)
    let red_parser = map(
        separated_pair(digit1, space1, tag("red")),
            |(digit, _): (&str, &str)| Color::Red(digit.parse().expect("Red was promised a u64 digit!"))
    );
    let red_parser = delimited(multispace0, red_parser, multispace0);

    let green_parser = map(
        separated_pair(digit1, space1, tag("green")),
        |(digit, _): (&str, &str)| Color::Green(digit.parse().expect("Green was promised a u64 digit!"))
    );
    let green_parser = delimited(multispace0, green_parser, multispace0);

    let blue_parser = map(
        separated_pair(digit1, space1, tag("blue")),
        |(digit, _): (&str, &str)| Color::Blue(digit.parse().expect("Blue was promised a u64 digit!"))
    );
    let blue_parser = delimited(multispace0, blue_parser, multispace0);

    let color_parser = alt((red_parser, green_parser, blue_parser));

    let set_parser = separated_list1(
        delimited(multispace0, char(','), multispace0),
        terminated(color_parser, multispace0)
    );

    // Step 3: Get each set for the game
    let (input, sets) = separated_list1(delimited(multispace0, char(';'), multispace0), set_parser)(input)?;

    Ok((input, Game { id, sets }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Game>> {
    many1(parse_line)(input)
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../data/input.txt").trim_end();

    let (_, games) = parse_lines(input)?;

    let sum: u64 = games
        .iter()
        .filter(|game| {
            game
            .sets
            .iter()
            .all(|set| {
                set
                .iter()
                .all(|color| {
                    match color {
                        Color::Red(n) => *n <= MAX_RED_CUBES,
                        Color::Green(n) => *n <= MAX_GREEN_CUBES,
                        Color::Blue(n) => *n <= MAX_BLUE_CUBES,
                    }
                })
            })
        })
        .map(|game| game.id)
        .sum();

    println!("{sum}");

    Ok(())
}
