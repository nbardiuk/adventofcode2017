use crate::intcode::Computer;

pub const INPUT: &str = include_str!("../res/day13.txt");

pub fn part1(input: &str) -> usize {
    Computer::parse(input)
        .run()
        .chunks(3)
        .filter(|tripple| match tripple[..] {
            [_, _, 2] => true,
            _ => false,
        })
        .count()
}

pub fn part2(input: &str) -> i64 {
    let mut computer = Computer::parse(input);
    computer.memory[0] = 2;
    let mut input = vec![];
    loop {
        let mut score = 0;
        let mut paddlex = 0;
        let mut ballx = 0;
        for tripple in computer.process(&mut input).chunks(3) {
            match tripple[..] {
                [-1, 0, s] => score = s,
                [x, _, 3] => paddlex = x,
                [x, _, 4] => ballx = x,
                _ => {}
            }
        }
        if computer.halted {
            return score;
        }
        input.push((ballx - paddlex).signum());
    }
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn part1_my_input() {
        assert_eq!(part1(INPUT), 200)
    }

    #[test]
    fn part2_my_input() {
        assert_eq!(part2(INPUT), 9803)
    }
}
