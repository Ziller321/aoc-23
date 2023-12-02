use std::fs;

struct Limit {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    println!("Day 2: ");

    let limit = Limit {
        red: 12,
        green: 13,
        blue: 14,
    };

    let path = "input/day2.txt";
    let contents = fs::read_to_string(path);

    match contents {
        Ok(res) => {
            let r = solve(res, limit);
            println!("{}", r)
        }
        Err(err) => {
            panic!("{}", err)
        }
    }
}

fn solve(s: String, limit: Limit) -> i32 {
    s.lines()
        .map(|x| {
            let game = x.split(": ").collect::<Vec<&str>>();
            let rounds = game[1]
                .split("; ")
                .filter_map(|r| {
                    match r
                        .split(", ")
                        .filter_map(|bb| {
                            let d = bb.split(" ").collect::<Vec<_>>();
                            let color = d[1];
                            let count = d[0].parse::<i32>().unwrap();

                            match color {
                                "red" => match count > limit.red {
                                    true => Some(count),
                                    false => None,
                                },
                                "green" => match count > limit.green {
                                    true => Some(count),
                                    false => None,
                                },
                                "blue" => match count > limit.blue {
                                    true => Some(count),
                                    false => None,
                                },
                                _ => None,
                            }
                        })
                        .peekable()
                        .peek()
                        .is_some()
                    {
                        true => Some(1),
                        false => None,
                    }
                })
                .peekable()
                .peek()
                .is_none();

            let kissa = game[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            if rounds {
                return kissa;
            }
            0
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let limit = Limit {
            red: 12,
            green: 13,
            blue: 14,
        };
        let result = solve(input.to_string(), limit);
        assert_eq!(result, 8);
    }
}
