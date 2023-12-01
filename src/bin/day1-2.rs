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

fn count_line(x: &str) -> i32 {
    let n: Vec<i32> = x
        .chars()
        .filter_map(|x| x.to_string().parse::<i32>().ok())
        .collect();

    if n.len() == 1 {
        10 * n[0] + n[0]
    } else if n.len() > 1 {
        10 * n[0] + n[n.len() - 1]
    } else {
        0
    }
}

fn parse_row(s: &str) -> i32 {
    let list: [(&str, Vec<_>, &str); 18] = [
        ("one", s.match_indices("one").collect(), "1"),
        ("two", s.match_indices("two").collect(), "2"),
        ("three", s.match_indices("three").collect(), "3"),
        ("four", s.match_indices("four").collect(), "4"),
        ("five", s.match_indices("five").collect(), "5"),
        ("six", s.match_indices("six").collect(), "6"),
        ("seven", s.match_indices("seven").collect(), "7"),
        ("eight", s.match_indices("eight").collect(), "8"),
        ("nine", s.match_indices("nine").collect(), "9"),
        ("1", s.match_indices("1").collect(), "1"),
        ("2", s.match_indices("2").collect(), "2"),
        ("3", s.match_indices("3").collect(), "3"),
        ("4", s.match_indices("4").collect(), "4"),
        ("5", s.match_indices("5").collect(), "5"),
        ("6", s.match_indices("6").collect(), "6"),
        ("7", s.match_indices("7").collect(), "7"),
        ("8", s.match_indices("8").collect(), "8"),
        ("9", s.match_indices("9").collect(), "9"),
    ];

    let mut re: Vec<(usize, &str)> = vec![];

    for (_, m, val) in list {
        for (idx, _) in m {
            re.push((idx, val))
        }
    }

    re.sort_by(|a, b| a.0.cmp(&b.0));

    let st = re
        .into_iter()
        .map(|(_, s)| s)
        .collect::<Vec<&str>>()
        .join("");

    count_line(&st.as_str())
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
