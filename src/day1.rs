// #[allow(dead_code)]
// const GROUND_FLOOR: i32 = 0;

#[aoc(day1, part1)]
// #[aoc_generator(day1)]
pub fn count_floors(input: &str) -> i32 {
    println!("Input:\n{}\n", input);
    let sum = input.chars().fold(0, |mut acc, c| {
        if c == '(' {
            acc = acc + 1;
        } else {
            acc = acc - 1;
        }
        acc
    });
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    // Floor with 3 down, 2 up, then 4 down
    fn test1() {
        use day1::count_floors;
        let result = count_floors(")))(())))");
        let correct_answer = -5;
        assert_eq!(result, correct_answer);
    }
}
