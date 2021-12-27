use anyhow::{anyhow, Error};
use std::fmt::{Display, Formatter, Write};
use std::str::{FromStr, Lines};

fn str_to_bools(line: &str) -> Vec<bool> {
    line.chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect()
}

struct Image {
    pixels: Vec<Vec<bool>>,
}
impl Image {
    fn count_lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|col| col.iter().filter(|p| **p).count())
            .sum()
    }
    fn dim(&self) -> (usize, usize) {
        match self.pixels.len() {
            0 => (0, 0),
            _ => (self.pixels.len(), self.pixels[0].len()),
        }
    }
    fn read_from(lines: Lines<'_>) -> Self {
        let pixels: Vec<Vec<bool>> = lines
            .filter(|l| !l.trim().is_empty())
            .map(str_to_bools)
            .collect();
        Self { pixels }
    }
    fn extract_centered(&self, edim: (usize, usize)) -> Self {
        let dim = self.dim();
        let offset = ((dim.0 - edim.0) / 2, (dim.1 - edim.1) / 2);

        Self {
            pixels: self
                .pixels
                .iter()
                .enumerate()
                .filter(|(j, _)| *j >= offset.1 && *j < offset.1 + edim.1)
                .map(|(_, line)| line[offset.0..edim.0 + offset.0].to_vec())
                .collect(),
        }
    }
}
impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in &self.pixels {
            for b in l {
                f.write_char(if *b { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

struct Enhancer {
    data: Vec<bool>,
}

impl Display for Enhancer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for b in &self.data {
            f.write_char(if *b { '#' } else { '.' })?;
        }
        f.write_char('\n')
    }
}
impl FromStr for Enhancer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = str_to_bools(s);
        if data.len() == 512 {
            return Ok(Self { data });
        }
        Err(anyhow!("extracted {} bits from '{}'", data.len(), s))
    }
}

impl Enhancer {
    fn get_3x3_lit_pixels(img: &Image, center: (isize, isize)) -> Vec<bool> {
        let dim = img.dim();
        (0..3)
            .flat_map(|j| {
                (0..3).map(move |i| {
                    if (center.1 + j >= 1 && center.1 + j < (dim.1 + 1) as isize)
                        && (center.0 + i >= 1 && center.0 + i < (dim.1 + 1) as isize)
                    {
                        img.pixels[(center.1 + j - 1) as usize][(center.0 + i - 1) as usize]
                    } else {
                        false
                    }
                })
            })
            .collect() // could provide a custom Iter instead
    }

    fn get_index_from_3x3_lit_pixels(img: &Image, center: (isize, isize)) -> usize {
        Self::get_3x3_lit_pixels(img, center)
            .iter()
            .fold(0, |acc, cur| acc * 2 + if *cur { 1 } else { 0 })
    }

    fn enhance(&self, img: &Image) -> Image {
        let dim = img.dim();
        Image {
            pixels: (-10..=dim.0 as isize + 9) // In case all black zones turns to white and vice-versa each step
                .map(|j| {
                    (-10..=dim.1 as isize + 9)
                        .map(|i| self.data[Self::get_index_from_3x3_lit_pixels(img, (i, j))])
                        .collect()
                })
                .collect(),
        }
    }
}
pub fn display_enhanced_img() {
    let input = include_str!("../ressources/day20_enhancer.txt");

    let mut lines = input.lines();

    let enhancer: Enhancer = lines.next().unwrap().parse().unwrap();
    let img = Image::read_from(lines);

    let mut new_img = img;
    for i in 1..=25 {
        // dta[0]=1 and data[9]=0 :
        // * at first step, all 0 surrounded by 0 are set to 1
        // * and go back to 0 the next step (if they are fully surrounded by 1s)
        // => the image can be cropped each 2 passes
        let dim = new_img.dim();
        new_img = enhancer
            .enhance(&enhancer.enhance(&new_img))
            .extract_centered((dim.0 + 8, dim.1 + 8));
        println!(
            "number of lit pixels after {} enhancements: {}",
            2 * i,
            new_img.count_lit_pixels()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Enhancer {
        fn count_3x3_lit_pixels(img: &Image, center: (isize, isize)) -> usize {
            Self::get_3x3_lit_pixels(img, center)
                .iter()
                .filter(|b| **b)
                .count()
        }
    }

    #[test]
    fn pixels_can_be_extracted() {
        let img = Image {
            pixels: vec![vec![true; 3], vec![false; 3], vec![true; 3]],
        };
        println!("{}", &img);

        assert_eq!(1, Enhancer::count_3x3_lit_pixels(&img, (-1, -1)));
        assert_eq!(2, Enhancer::count_3x3_lit_pixels(&img, (0, 0)));
        assert_eq!(6, Enhancer::count_3x3_lit_pixels(&img, (1, 1)));
        assert_eq!(0, Enhancer::count_3x3_lit_pixels(&img, (4, 1)));
        assert_eq!(2, Enhancer::count_3x3_lit_pixels(&img, (3, 1)));

        assert_eq!(1, Enhancer::get_index_from_3x3_lit_pixels(&img, (-1, -1)));
        /// 111000111
        assert_eq!(455, Enhancer::get_index_from_3x3_lit_pixels(&img, (1, 1)));

        // 000011000
        assert_eq!(24, Enhancer::get_index_from_3x3_lit_pixels(&img, (0, 0)));

        // 111000000
        assert_eq!(448, Enhancer::get_index_from_3x3_lit_pixels(&img, (1, 3)));
        // 100000100
        assert_eq!(260, Enhancer::get_index_from_3x3_lit_pixels(&img, (3, 1)));
    }

    #[test]
    fn aoc_example_work() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let mut lines = input.lines();
        assert_eq!(7, lines.clone().count());

        let enhancer: Enhancer = lines.next().unwrap().parse().unwrap();
        let img = Image::read_from(lines);

        assert_eq!(34, Enhancer::get_index_from_3x3_lit_pixels(&img, (2, 2)));
        assert_eq!(false, enhancer.data[0]);
        assert_eq!(true, enhancer.data[34]);
        assert_eq!(true, enhancer.data[50]);
        assert_eq!((5, 5), img.dim());
        println!("original\n{}", &img);

        assert_eq!(
            35,
            enhancer.enhance(&enhancer.enhance(&img)).count_lit_pixels()
        );
        let mut img = img;
        for _ in 0..50 {
            let dim = img.dim();
            img = enhancer
                .enhance(&img)
                .extract_centered((dim.0 + 4, dim.1 + 4));
        }
        println!("after 50 steps\n{}", &img);
        assert_eq!(3351, img.count_lit_pixels());
    }
}
