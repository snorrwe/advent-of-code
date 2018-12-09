use std::collections::BTreeMap;

fn main() {
    let part1 = play(464, 70918);
    println!("Part 1: {}", part1);
    // let part2 = play(464, 70918 * 100);
    // println!("Part 2: {}", part2);
}

fn play(n_players: usize, n_marbles: usize) -> usize {
    let mut board = Vec::with_capacity(n_marbles);
    board.push(0);
    let mut current_ind = 0;
    let mut current_player = 0;
    let mut scores = BTreeMap::new();
    for marble in 1..=n_marbles {
        if marble % 23 == 0 {
            let score = scores.entry(current_player).or_insert(0);
            *score += marble;
            current_ind = rotate_counter_clockwise(current_ind, -7, board.len());
            *score += board.remove(current_ind);
        } else {
            current_ind = rotate_counter_clockwise(current_ind, 2, board.len());
            board.insert(current_ind, marble);
        }
        current_player = (current_player + 1) % n_players;
    }
    scores.values().max().map_or(0, |x| *x)
}

fn rotate_counter_clockwise(index: usize, steps: i64, len: usize) -> usize {
    let index = index as i64;
    let len = len as i64;
    let mut result = (index + steps) % len;
    if result < 0 {
        result += len
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

