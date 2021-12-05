#![feature(test)]
extern crate test;

use std::fs::read_to_string;

fn part1(image: &str, width: usize, height: usize) -> i32 {
    image
        .char_indices()
        .filter(|(_, c)| c.is_numeric())
        .step_by(width * height)
        .map(|(i, _)| {
            image[i..i + (width * height)]
                .chars()
                .fold((0i32, 0i32, 0i32), |mut res, ch| {
                    match ch {
                        '0' => res.0 += 1,
                        '1' => res.1 += 1,
                        '2' => res.2 += 1,
                        _ => {}
                    }
                    res
                })
        })
        .map(|(zeros, ones, twos)| (zeros, ones * twos))
        .min_by_key(|(z, _)| *z)
        .map(|(_, v)| v)
        .expect("min")
}

/// print an image
fn part2(image: &str, width: usize, height: usize) {
    let image = image
        .char_indices()
        .filter(|(_, c)| c.is_numeric())
        .step_by(width * height)
        .map(|(i, _)| &image[i..i + (width * height)])
        .fold(vec!['2'; width * height], |mut image, layer| {
            for (i, c) in layer.char_indices() {
                if image[i] != '2' {
                    continue;
                }
                image[i] = c;
            }
            image
        });
    print_image(&image, width, height);
}

fn print_image(image: &[char], width: usize, height: usize) {
    for h in 0..height {
        let h = h * width;
        println!(
            "{}",
            image[h..h + width]
                .iter()
                .cloned()
                .map(|c| match c {
                    '0' => ' ',
                    '1' => '.',
                    _ => c,
                })
                .collect::<String>()
        );
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let res = part1(&input, 25, 6);
    println!("{:?}", res);
    part2(&input, 25, 6);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| main());
    }
}
