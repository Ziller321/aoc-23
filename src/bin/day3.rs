use std::{collections::HashMap, fs};

fn main() {
    println!("Day 1");

    let path = "input/day3.txt";
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

fn solve(input: String) -> i32 {
    p(input.as_str());
    0
}

fn find_n(row: i32, start: i32, end: i32) -> Vec<(i32, i32)> {
    let left = vec![
        (-1 + start, -1 + row),
        (-1 + start, 0 + row),
        (-1 + start, 1 + row),
    ];

    let right = vec![(1 + end, -1 + row), (1 + end, 0 + row), (1 + end, 1 + row)];

    let middle: Vec<(i32, i32)> = (0..(end - start + 1))
        .into_iter()
        .map(|x| {
            // println!("{:?}", x);
            vec![(x + start, -1 + row), (x + start, 1 + row)]
        })
        .flatten()
        .collect();

    let all: Vec<(i32, i32)> = vec![left, right, middle].into_iter().flatten().collect();

    // println!("Middle {:?}", all);
    all
}

fn p(s: &str) -> i32 {
    let mut lx: usize = 0;
    let mut rx: usize = 0;
    let mut num = String::from("");
    let mut sum: i32 = 0;
    let mut part_map: HashMap<(i32, i32), String> = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if !char.is_numeric() && char != '.' {
                part_map.insert(
                    (x.try_into().unwrap(), y.try_into().unwrap()),
                    char.to_string(),
                );
            }
        }
    }

    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char.to_digit(10) {
                Some(val) => {
                    if num.len() == 0 {
                        lx = x;
                    }
                    num.push_str(&val.to_string().as_str());
                    rx = x;
                }
                None => {
                    // Break in number
                    if num.len() > 0 {
                        let nn = find_n(
                            y.try_into().unwrap(),
                            lx.try_into().unwrap(),
                            rx.try_into().unwrap(),
                        );

                        for n in nn {
                            match part_map.get(&n) {
                                None => (),
                                Some(part_icon) => {
                                    println!("Num: {} is {} at {:?}", num, part_icon, n);
                                    let pn: i32 = num.parse().unwrap();

                                    sum += pn;
                                }
                            };
                        }
                    }
                    num = String::from("");
                }
            };
        }
    }

    println!("{}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(p(input), 4361);
    }
}
