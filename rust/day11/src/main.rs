fn main() {
    let part1 = largest_grid(7689, 3);
    println!("Part1: {:?}", part1);
}

fn largest_grid(serial: i32, grid_size: i32) -> (i32, i32) {
    let mut max_coord = (0, 0);
    let mut max_value = 0;
    for x in 1..=300 - grid_size {
        for y in 1..=300 - grid_size {
            let mut value = 0;
            for i in 0..grid_size {
                for j in 0..grid_size {
                    value += power_level(x + i, y + j, serial);
                }
            }
            if value > max_value {
                max_value = value;
                max_coord = (x, y);
            }
        }
    }
    max_coord
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

        assert_eq!(actual, (33, 45));
    }

}

