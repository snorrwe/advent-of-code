use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

enum WinCondition {
    Win,
    Lose,
    Draw,
}

impl FromStr for WinCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "X" => WinCondition::Lose,
            "Y" => WinCondition::Draw,
            "Z" => WinCondition::Win,
            _ => return Err(()),
        };
        Ok(res)
    }
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

    /// what's the score of the item that beats self
    pub fn score_beat(self) -> i32 {
        match self {
            RockPaperScissor::Rock => 2,
            RockPaperScissor::Paper => 3,
            RockPaperScissor::Scissor => 1,
        }
    }

    /// what's the score of the item that looses to self
    pub fn score_loose(self) -> i32 {
        match self {
            RockPaperScissor::Rock => 3,
            RockPaperScissor::Paper => 1,
            RockPaperScissor::Scissor => 2,
        }
    }
}

fn part1(input: &str) -> i32 {
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

fn part2(input: &str) -> i32 {
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
        let win = WinCondition::from_str(columns.next().unwrap()).unwrap();
        match win {
            WinCondition::Win => {
                score += 6 + opp.score_beat();
            }
            WinCondition::Lose => score += opp.score_loose(),
            WinCondition::Draw => {
                score += 3 + opp.score();
            }
        }
    }
    score
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let p1 = part1(&input);
    println!("p1: {}", p1);

    let p2 = part2(&input);
    println!("p2: {}", p2);
}

#[test]
fn part1_test() {
    let result = part1(
        r#"A Y
B X
C Z
"#,
    );

    assert_eq!(result, 15)
}

#[test]
fn part2_test() {
    let result = part2(
        r#"A Y
B X
C Z
"#,
    );

    assert_eq!(result, 12)
}
