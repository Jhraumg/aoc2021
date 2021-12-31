use crate::day19::ThreeDPoint;
use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::cmp::{max, min};
use std::ops::Range;
use std::str::FromStr;

struct Reactor<const SIZE: usize> {
    offset: usize,
    cubes: Vec<Vec<Vec<bool>>>,
}

impl<const SIZE: usize> Reactor<SIZE> {
    fn new() -> Self {
        Self {
            offset: (SIZE - 1) / 2,
            cubes: vec![vec![vec![false; SIZE]; SIZE]; SIZE],
        }
    }

    fn get_mut_cube(&mut self, coords: &ThreeDPoint) -> Option<&mut bool> {
        let translated_coords = ThreeDPoint {
            x: coords.x + self.offset as isize,
            y: coords.y + self.offset as isize,
            z: coords.z + self.offset as isize,
        };

        for coord in [
            translated_coords.x,
            translated_coords.y,
            translated_coords.z,
        ] {
            if coord < 0 || coord >= SIZE as isize {
                return None;
            }
        }

        Some(
            &mut self.cubes[translated_coords.x as usize][translated_coords.y as usize]
                [translated_coords.z as usize],
        )
    }

    fn count_lit_cubes(&self) -> usize {
        self.cubes
            .iter()
            .map(|dslice| {
                dslice
                    .iter()
                    .map(|v| v.iter().filter(|c| **c).count())
                    .sum::<usize>()
            })
            .sum()
    }
}

fn intersect_vrange(r1: &(isize, isize), r2: &(isize, isize)) -> Option<(isize, isize)> {
    let rmin = min(max(r1.0, r2.0), r2.1 + 1);
    let rmax = max(r2.0 - 1, min(r1.1, r2.1));

    if rmax >= rmin {
        Some((rmin, rmax))
    } else {
        None
    }
}

fn get_actual_range(proposed: &(isize, isize), possible: &(isize, isize)) -> Range<isize> {
    if let Some((rmin, rmax)) = intersect_vrange(proposed, possible) {
        rmin..(rmax + 1)
    } else {
        0..0
    }
}

#[derive(Debug, Clone, Default)]
struct Cuboid {
    xrange: (isize, isize),
    yrange: (isize, isize),
    zrange: (isize, isize),
}

impl Cuboid {
    fn intersect(&self, other: &Self) -> Option<Self> {
        if let Some((xrange, yrange, zrange)) = (0..3)
            .filter_map(|i| intersect_vrange(self.get_range(i), other.get_range(i)))
            .collect_tuple()
        {
            Some(Self {
                xrange,
                yrange,
                zrange,
            })
        } else {
            None
        }
    }

    fn get_range(&self, i: usize) -> &(isize, isize) {
        match i {
            0 => &self.xrange,
            1 => &self.yrange,
            2 => &self.zrange,

            _ => panic!("unknown {} range : a Cuboid holds only 3 ranges (0..=2)", i),
        }
    }

    fn size(&self) -> usize {
        (self.xrange.1 - self.xrange.0 + 1) as usize
            * (self.yrange.1 - self.yrange.0 + 1) as usize
            * (self.zrange.1 - self.zrange.0 + 1) as usize
    }
}

struct Command {
    status: bool,
    cubes: Cuboid,
}

impl Command {
    // tbh, using &mut dyn Reactor (without size) should be more practicable
    fn apply<const SIZE: usize>(&self, reactor: &mut Reactor<SIZE>) {
        let possible = (-(reactor.offset as isize), reactor.offset as isize);
        for x in get_actual_range(&self.cubes.xrange, &possible) {
            for y in get_actual_range(&self.cubes.yrange, &possible) {
                for z in get_actual_range(&self.cubes.zrange, &possible) {
                    if let Some(cube) = reactor.get_mut_cube(&ThreeDPoint { x, y, z }) {
                        *cube = self.status;
                    }
                }
            }
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut status: Option<bool> = None;
        let mut xrange: Option<(isize, isize)> = None;
        let mut yrange: Option<(isize, isize)> = None;
        let mut zrange: Option<(isize, isize)> = None;
        for v in input.split(&[' ', ','][..]) {
            match v {
                status_str if ["on", "off"].contains(&status_str) => {
                    status = Some(status_str == "on");
                }
                range if range.contains("..") => {
                    if let Some((axis, range)) = range.split('=').collect_tuple() {
                        if let Some((rmin, rmax)) = range.split("..").collect_tuple() {
                            let rmin: isize = rmin.parse()?;
                            let rmax: isize = rmax.parse()?;
                            let range = Some((rmin, rmax));

                            match axis {
                                "x" => {
                                    xrange = range;
                                }
                                "y" => {
                                    yrange = range;
                                }
                                "z" => {
                                    zrange = range;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if status.and(xrange).and(yrange).and(zrange).is_some() {
            let status = status.unwrap();
            let xrange = xrange.unwrap();
            let yrange = yrange.unwrap();
            let zrange = zrange.unwrap();
            Ok(Command {
                status,
                cubes: Cuboid {
                    xrange,
                    yrange,
                    zrange,
                },
            })
        } else {
            Err(anyhow!("could not produce Command from '{}'", input))
        }
    }
}

// Naïve approach : all cubes are represented by a bool !
fn reboot_sized_reactor(input: &str) -> usize {
    let mut reactor: Reactor<101> = Reactor::new();
    let commands: Vec<Command> = input
        .lines()
        .filter_map(|l| l.parse::<Command>().ok())
        .collect();

    for command in &commands {
        command.apply(&mut reactor);
    }
    reactor.count_lit_cubes()
}

fn reboot_unsized_reactor(input: &str) -> usize {
    let commands: Vec<Command> = input
        .lines()
        .filter_map(|l| l.parse::<Command>().ok())
        .collect();
    let mut commands_counting_overlap: Vec<Command> = Vec::with_capacity(commands.len());
    for command in commands {
        let mut extra_command: Vec<Command> = Vec::with_capacity(commands_counting_overlap.len());

        // all ON commands are added to the mix
        // * extra OFF commands are added for intersections with already present ON commands
        // * extra ON commands are added for intersections with already present OFF commands
        // OFF commands are not added
        // * extra OFF commands are added for each intersection with already present ON commands
        // * extra ON commands are added for each intersection with already present OFF commands
        for c in &commands_counting_overlap {
            if let Some(cubes) = c.cubes.intersect(&command.cubes) {
                let status = match (command.status, c.status) {
                    (false, false) => true, // OFF ∩ OFF => ON
                    (false, true) => false, // OFF ∩ ON => OFF
                    (true, false) => true,  // ON ∩ OFF => ON
                    (true, true) => false,  // ON ∩ ON => OFF
                };
                extra_command.push(Command { status, cubes });
            }
        }
        if command.status {
            extra_command.push(command);
        }
        commands_counting_overlap.append(&mut extra_command);
    }
    commands_counting_overlap
        .iter()
        .map(|c| {
            if c.status {
                c.cubes.size() as isize
            } else {
                0 - c.cubes.size() as isize
            }
        })
        .sum::<isize>() as usize
}

pub fn display_reactor_reboot() {
    let input = include_str!("../ressources/day22_reboot_sequence.txt");
    println!(
        "number of cubes lit after reboot sequence for 101*101*101 reactor : {}",
        reboot_sized_reactor(input)
    );
    println!(
        "number of cubes lit after reboot sequence for full reactor : {}",
        reboot_unsized_reactor(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_ranges_can_always_be_extracted() {
        assert_eq!(101, get_actual_range(&(-50, 50), &(-50, 50)).len());
        assert_eq!(101, get_actual_range(&(-50, 50), &(-50, 50)).len());
        assert_eq!(1, get_actual_range(&(0, 0), &(-50, 50)).len());
        assert_eq!(101, get_actual_range(&(-100, 100), &(-50, 50)).len());
        assert_eq!(0, get_actual_range(&(-100, -51), &(-50, 50)).len());
        assert_eq!(1, get_actual_range(&(-100, -50), &(-50, 50)).len());
        assert_eq!(0, get_actual_range(&(51, 1000), &(-50, 50)).len());
        assert_eq!(1, get_actual_range(&(50, 1000), &(-50, 50)).len());

        assert_eq!(0, get_actual_range(&(-100, -101), &(-100, 0)).len());
    }

    #[test]
    fn simple_aoc_example_works() {
        let input = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
        let commands: Vec<_> = input
            .lines()
            .filter_map(|l| l.parse::<Command>().ok())
            .collect();
        let mut reactor: Reactor<31> = Reactor::new();
        for c in commands {
            c.apply(&mut reactor);
        }
        assert_eq!(39, reactor.count_lit_cubes());
    }
    #[test]
    fn aoc_example_works() {
        let input = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
        assert_eq!(590784, reboot_sized_reactor(input));
        assert_eq!(2758514936282235, reboot_unsized_reactor(input));
    }
}
