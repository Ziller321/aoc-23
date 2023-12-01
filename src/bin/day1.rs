use std::fs;

fn main() {
    println!("Day 1");

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

fn solve(input: String) -> i32 {
    input
        .lines()
        .map(|x| {
            let n: Vec<i32> = x
                .chars()
                .filter_map(|x| x.to_string().parse::<i32>().ok())
                .collect();
            if n.len() == 1 {
                10 * n[0] + n[0]
            } else {
                10 * n[0] + n[n.len() - 1]
            }
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let result = solve(input.to_string());
        assert_eq!(result, 142);
    }
}
