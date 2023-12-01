use std::fs;

fn main() {
    println!("Day 1 - 2");

    let path = "input/day1.txt";
    let contents = fs::read_to_string(path);

    match contents {
        Ok(res) => {
            let r = solve(res);
            println!("{}", r)
        }
        Err(err) => {
            panic!("{}", err)
        }
    }
}

fn count_line(x: String) -> i32 {
    let n: Vec<i32> = x
        .chars()
        .filter_map(|x| x.to_string().parse::<i32>().ok())
        .collect();

    let first = n.first().unwrap();
    let last = n.last().unwrap();

    match n.len() {
        1 => 10 * first + first,
        0 => 0,
        _ => 10 * first + last,
    }
}

fn parse_row(s: &str) -> i32 {
    let list: [(Vec<_>, &str); 18] = [
        (s.match_indices("one").collect(), "1"),
        (s.match_indices("two").collect(), "2"),
        (s.match_indices("three").collect(), "3"),
        (s.match_indices("four").collect(), "4"),
        (s.match_indices("five").collect(), "5"),
        (s.match_indices("six").collect(), "6"),
        (s.match_indices("seven").collect(), "7"),
        (s.match_indices("eight").collect(), "8"),
        (s.match_indices("nine").collect(), "9"),
        (s.match_indices("1").collect(), "1"),
        (s.match_indices("2").collect(), "2"),
        (s.match_indices("3").collect(), "3"),
        (s.match_indices("4").collect(), "4"),
        (s.match_indices("5").collect(), "5"),
        (s.match_indices("6").collect(), "6"),
        (s.match_indices("7").collect(), "7"),
        (s.match_indices("8").collect(), "8"),
        (s.match_indices("9").collect(), "9"),
    ];

    let mut result: Vec<(usize, &str)> = vec![];

    for (m, val) in list {
        for (idx, _) in m {
            result.push((idx, val))
        }
    }

    result.sort_by(|a, b| a.0.cmp(&b.0));

    let str_list = result
        .into_iter()
        .map(|(_, s)| s)
        .collect::<Vec<&str>>()
        .join("");

    count_line(str_list)
}

fn solve(input: String) -> i32 {
    input.lines().map(|x| parse_row(x)).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        assert_eq!(parse_row("two1nine"), 29);
        assert_eq!(parse_row("eightwothree"), 83);
        assert_eq!(parse_row("abcone2threexyz"), 13);
        assert_eq!(parse_row("xtwone3four"), 24);
        assert_eq!(parse_row("4nineeightseven2"), 42);
        assert_eq!(parse_row("zoneight234"), 14);
        assert_eq!(parse_row("7pqrstsixteen"), 76);
        assert_eq!(parse_row("seven7seven"), 77);

        assert_eq!(parse_row("eighthree"), 83);
        assert_eq!(parse_row("sevenine"), 79);
    }

    #[test]
    fn it_works() {
        let input = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;
        let result = solve(input.to_string());
        assert_eq!(result, 281);
    }
}
