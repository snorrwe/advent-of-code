#[derive(Clone, Copy, Debug)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl RockPaperScissor {
    pub fn beats(self, rhs: Self) -> bool {
        match (self, rhs) {
            (RockPaperScissor::Rock, RockPaperScissor::Scissor)
            | (RockPaperScissor::Paper, RockPaperScissor::Rock)
            | (RockPaperScissor::Scissor, RockPaperScissor::Paper) => true,
            _ => false,
        }
    }

    pub fn score(self) -> i32 {
        match self {
            RockPaperScissor::Rock => 1,
            RockPaperScissor::Paper => 2,
            RockPaperScissor::Scissor => 3,
        }
    }
}

fn solve(input: &str) -> i32 {
    let mut score = 0;
    for row in input.lines() {
        let row = row.trim();
        let mut columns = row.split(" ");
        let opp = match columns.next().unwrap() {
            "A" => RockPaperScissor::Rock,
            "B" => RockPaperScissor::Paper,
            "C" => RockPaperScissor::Scissor,
            _ => unreachable!(),
        };
        let mine = match columns.next().unwrap() {
            "X" => RockPaperScissor::Rock,
            "Y" => RockPaperScissor::Paper,
            "Z" => RockPaperScissor::Scissor,
            _ => unreachable!(),
        };
        score += mine.score();
        if mine.beats(opp) {
            score += 6;
        } else if !opp.beats(mine) {
            score += 3;
        }
    }
    score
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let p1 = solve(&input);
    println!("p1: {}", p1);
}

#[test]
fn part1_test() {
    let result = solve(
        r#"A Y
B X
C Z
"#,
    );

    assert_eq!(result, 15)
}
