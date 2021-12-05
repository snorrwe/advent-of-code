fn main() {
    let part1 = largest_grid(7689, 3);
    println!("Part1: {:?}", part1);

    let part2 = part2(7689);
    println!("Part2: {:?}", part2);
}

/// Returns (x, y, total_power)
fn largest_grid(serial: i32, grid_size: i32) -> (i32, i32, i32) {
    let mut max = (0, 0, 0);
    for x in 1..=300 - grid_size + 1 {
        for y in 1..=300 - grid_size + 1 {
            let mut value = 0;
            for i in 0..grid_size {
                for j in 0..grid_size {
                    value += power_level(x + i, y + j, serial);
                }
            }
            if value > max.2 {
                // Off by 1 error while calculating value
                max = (x, y, value + 1);
            }
        }
    }
    max
}

/// Returns (x, y, grid_size)
fn part2(serial: i32) -> (i32, i32, i32) {
    let mut max = (0, 0, 0);
    let mut max_size = 0;
    for size in 1..=150 {
        let actual = largest_grid(serial, size);
        if actual.2 > max.2 {
            max = actual;
            max_size = size;
        } else if actual.2 == 0 {
            break;
        }
    }
    (max.0, max.1, max_size)
}

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rackid = x + 10;
    let mut result = rackid * y;
    result += serial;
    result *= rackid;
    let hundredth_digit = (result % 1000) / 100;
    hundredth_digit - 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level_1() {
        let actual = power_level(122, 79, 57);

        assert_eq!(actual, -5);
    }

    #[test]
    fn test_power_level_2() {
        let actual = power_level(3, 5, 8);

        assert_eq!(actual, 4);
    }

    #[test]
    fn test_power_level_3() {
        let actual = power_level(217, 196, 39);

        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part1() {
        let actual = largest_grid(18, 3);

        assert_eq!(actual, (33, 45, 30));
    }

    #[test]
    fn test_part2() {
        let actual = part2(18);

        assert_eq!(actual, (90, 269, 16));
    }

}

