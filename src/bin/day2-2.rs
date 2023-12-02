use std::fs;

fn main() {
    println!("Day 2: ");

    let path = "input/day2.txt";
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

fn solve(s: String) -> i32 {
    s.lines()
        .map(|x| {
            let game = x.split(": ").collect::<Vec<&str>>();
            let balls = get_balls(game[1]);
            println!("=< ball {:?}", balls);

            balls.0 * balls.1 * balls.2

            // 0
        })
        .sum::<i32>()
}

type RGB = (i32, i32, i32);

fn get_balls(s: &str) -> RGB {
    let b = s
        .split("; ")
        .map(|x| parse_set(x))
        .fold((0, 0, 0), |acc, x| {
            let ((_, r), (_, g), (_, b)) = x;

            let (mut ar, mut ag, mut ab) = acc;

            if r > ar {
                ar = r;
            }
            if g > ag {
                ag = g;
            }
            if b > ab {
                ab = b;
            }

            println!("x: {:?}", x);
            (ar, ag, ab)
        });

    b
}

fn parse_ball(s: &str) -> (&str, i32) {
    let d = s.split(" ").collect::<Vec<&str>>();
    (d[1], d[0].parse::<i32>().unwrap())
}

fn parse_set(s: &str) -> ((&str, i32), (&str, i32), (&str, i32)) {
    s.split(", ")
        .map(|x| parse_ball(x))
        .fold((("red", 0), ("green", 0), ("blue", 0)), |acc, x| {
            let mut kissa = acc;

            match x.0 {
                "red" => kissa.0 .1 += x.1,
                "green" => kissa.1 .1 += x.1,
                "blue" => kissa.2 .1 += x.1,
                _ => (),
            };
            kissa
        })
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

        // let result = solve(input.to_string(), limit)
        // assert_eq!(result, 8);
        //

        assert_eq!(parse_ball("3 blue"), ("blue", 3));

        assert_eq!(
            parse_set("3 blue, 1 green"),
            (("red", 0), ("green", 1), ("blue", 3))
        );

        assert_eq!(
            get_balls("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (4, 2, 6)
        );

        assert_eq!(solve(input.to_string()), 2286)
    }
}
