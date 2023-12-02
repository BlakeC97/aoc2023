fn main() {
    let input = include_str!("../data/input.txt").trim_end();

    let sum: u32 = input
        .lines()
        .map(|line| {
            let (mut first_num, mut last_num) = (0, 0);

            line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .for_each(|c| {
                    last_num = c;
                    if first_num == 0 {
                        first_num = c;
                    }
                });

            (first_num * 10) + last_num
        })
        .sum();

    println!("{sum}");
}
