extern crate pbr;

use pbr::ProgressBar;
use std::collections::{BTreeMap, VecDeque};

fn main() {
    play(464, 70_918);
    play(464, 7_091_800);
}

fn play(n_players: usize, n_marbles: usize) -> usize {
    let mut board = VecDeque::with_capacity(n_marbles);
    board.push_back(0);
    let mut current_ind = 0;
    let mut scores = BTreeMap::new();
    let mut pb = ProgressBar::new(n_marbles as u64);
    for marble in 1..=n_marbles {
        if marble % 23 == 0 {
            let current_player = rotate_counter_clockwise(marble % n_players, -1, n_players);
            let score = scores.entry(current_player).or_insert(0);
            *score += marble;
            current_ind = rotate_counter_clockwise(current_ind, -7, board.len());
            let removed = board
                .remove(current_ind)
                .expect("Attempting to remove non existing index");
            *score += removed;
        } else {
            current_ind = rotate_counter_clockwise(current_ind, 2, board.len());
            board.insert(current_ind, marble);
        }
        if marble % 32_768 == 0 {
            pb.set(marble as u64);
        }
    }
    let result = scores.values().max().map_or(0, |x| *x);
    pb.finish_print(&format!(
        "\r\nDone\nn_players: {}\nn_marbles: {}\nresult: {}\r\n",
        n_players, n_marbles, result
    ));
    result
}

fn rotate_counter_clockwise(index: usize, steps: i64, len: usize) -> usize {
    let index = index as i64;
    let len = len as i64;
    let mut result = (index + steps) % len;
    if result < 0 {
        result += len;
    }
    result as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_play_9_players_25_marbles() {
        let result = play(9, 25);

        assert_eq!(result, 32);
    }

    #[test]
    fn test_play_10_players_1618_marbles() {
        let result = play(10, 1618);

        assert_eq!(result, 8317);
    }

    #[test]
    fn test_play_13_players_7999_marbles() {
        let result = play(13, 7999);

        assert_eq!(result, 146373);
    }
}

