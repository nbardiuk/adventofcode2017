use crate::intcode::Computer;
pub const INPUT: &str = include_str!("../res/day21.txt");

pub fn part1(input: &str) -> i64 {
    let program = "\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";
    Computer::parse(input).call(program.chars().map(|c| (c as u8) as i64).collect())
}

#[cfg(test)]
mod spec {
    use super::*;
    #[test]
    fn part1_my_input() {
        assert_eq!(part1(INPUT), 19358416);
    }
}
