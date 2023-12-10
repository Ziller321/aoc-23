use std::fs;

fn main() {
    println!("Day 1");

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

fn solve(input: &str) -> i32 {
    input.lines().map(|line| solve_row(line)).sum::<i32>()
}

fn solve_row(s: &str) -> i32 {
    let mut parts = s.split(&[':', '|']).skip(1);

    let winning = parts.next().unwrap().split_ascii_whitespace();
    let my_numbers = parts.next().unwrap().split_ascii_whitespace();

    let count_win: u32 = winning
        .clone()
        .filter_map(|num| my_numbers.clone().find(|&x| x == num))
        .collect::<Vec<&str>>()
        .len() as u32;

    if count_win == 0 {
        0
    } else {
        2_i32.pow(count_win - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve_row("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            solve_row("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            solve_row("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            solve_row("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            solve_row("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            solve_row("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
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
            13
        );
    }
}
