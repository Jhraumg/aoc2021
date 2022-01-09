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
    let input = include_str!("../resources/day22_reboot_sequence.txt");
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
    fn aoc_sized_cube_example_works() {
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
    }
    #[test]
    fn aoc_unsized_cube_example_works() {
        let input = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
        assert_eq!(2758514936282235, reboot_unsized_reactor(input));
    }
}
