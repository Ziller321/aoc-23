use std::{collections::HashMap, fs, ops::RangeInclusive};

fn main() {
    println!("Day 4-2");

    let path = "input/day4.txt";
    let contents = fs::read_to_string(path);

    match contents {
        Ok(res) => {
            let r = solve(&res);
            println!("{}", r)
        }
        Err(err) => {
            panic!("{}", err)
        }
    }
}

type Cache = HashMap<usize, i32>;

fn solve(input: &str) -> i32 {
    let mut cache: Cache = HashMap::new();
    let c: Vec<&str> = input.lines().collect();

    (1..=c.len())
        .map(|nn| count_row(nn, &c, &mut cache))
        .sum::<i32>()
}

fn count_row(game: usize, rows: &Vec<&str>, c: &mut Cache) -> i32 {
    if c.contains_key(&game) {
        return *c.get(&game).unwrap();
    }

    let n: i32 = solve_row(rows.get(game - 1).unwrap())
        .map(|i| {
            if i as usize != game {
                return count_row(i as usize, rows, c);
            } else {
                return 1;
            }
        })
        .sum();

    c.insert(game, n);

    n
}

fn solve_row(s: &str) -> RangeInclusive<i32> {
    let mut parts = s.split(&[':', '|']);

    let game = parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let winning = parts.next().unwrap().split_ascii_whitespace();
    let my_numbers = parts.next().unwrap().split_ascii_whitespace();

    let count_win: u32 = winning
        .clone()
        .filter_map(|num| my_numbers.clone().find(|&x| x == num))
        .collect::<Vec<&str>>()
        .len() as u32;

    game..=game + count_win as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve_row("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            1..=5
        );

        assert_eq!(
            solve_row("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2..=4
        );
        assert_eq!(
            solve_row("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            3..=5
        );
        assert_eq!(
            solve_row("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            4..=5
        );
        assert_eq!(
            solve_row("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            5..=5
        );
        assert_eq!(
            solve_row("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            6..=6
        );

        assert_eq!(
            solve(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }
}
