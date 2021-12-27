#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct ThreeDPoints {
    x: isize,
    y: isize,
    z: isize,
}
impl FromStr for ThreeDPoints {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y, z)) = s
            .split(',')
            .filter_map(|v| v.parse::<isize>().ok())
            .collect_tuple()
        {
            return Ok(Self { x, y, z });
        }
        Err(anyhow!("could not read 3D point from '{}'", s))
    }
}

#[derive(Debug, Clone, Copy)]
struct Vect {
    x: isize,
    y: isize,
    z: isize,
}

impl std::ops::Sub<ThreeDPoints> for ThreeDPoints {
    type Output = Vect;

    fn sub(self, rhs: ThreeDPoints) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Fixme : define Vector, etc...
impl std::ops::Add<Vect> for ThreeDPoints {
    type Output = ThreeDPoints;
    fn add(self, rhs: Vect) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    // type Output = ThreeDPoints;
    //
    // fn add(self, rhs: Self) -> Self::Output {
    //     Self::Output{x:self.x + rhs.x, y:self.y + rhs.y, z:self.z+rhs.z}
    // }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
}

use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;
use Direction::{RX, RY, RZ, X, Y, Z};
impl Direction {
    fn get_unary_coordinate(&self, point: &ThreeDPoints) -> isize {
        match self {
            X => point.x,
            Y => point.y,
            Z => point.z,
            RX => -point.x,
            RY => -point.y,
            RZ => -point.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ScanDir {
    facing: Direction,
    up: Direction,
}
impl ScanDir {
    fn get_all_scandirs() -> &'static [ScanDir; 24] {
        static SCAN_DIRS: [ScanDir; 24] = [
            ScanDir { facing: X, up: Z },
            ScanDir { facing: X, up: RZ },
            ScanDir { facing: X, up: Y },
            ScanDir { facing: X, up: RY },
            ScanDir { facing: RX, up: Z },
            ScanDir { facing: RX, up: RZ },
            ScanDir { facing: RX, up: Y },
            ScanDir { facing: RX, up: RY },
            ScanDir { facing: Y, up: X },
            ScanDir { facing: Y, up: RX },
            ScanDir { facing: Y, up: Z },
            ScanDir { facing: Y, up: RZ },
            ScanDir { facing: RY, up: X },
            ScanDir { facing: RY, up: RX },
            ScanDir { facing: RY, up: Z },
            ScanDir { facing: RY, up: RZ },
            ScanDir { facing: Z, up: X },
            ScanDir { facing: Z, up: RX },
            ScanDir { facing: Z, up: Y },
            ScanDir { facing: Z, up: RY },
            ScanDir { facing: RZ, up: X },
            ScanDir { facing: RZ, up: RX },
            ScanDir { facing: RZ, up: Y },
            ScanDir { facing: RZ, up: RY },
        ];
        &SCAN_DIRS
    }

    // return a tuple indicating
    fn get_inverse_axes(&self) -> Result<(Direction, Direction, Direction), Error> {
        match (self.facing, self.up) {
            (X, Z) => Ok((X, Y, Z)), //ThreeDPoints{x:1,y:2,z:3}
            (X, RZ) => Ok((X, RY, RZ)),
            (X, Y) => Ok((X, Z, RY)),
            (X, RY) => Ok((X, RZ, Y)),    //ThreeDPoints{x:1,y:3,z:-2}
            (RX, Z) => Ok((RX, RY, Z)),   //ThreeDPoints{x:-1,y:-2,z:3}
            (RX, RZ) => Ok((RX, Y, RZ)),  //ThreeDPoints{x:-1,y:2,z:-3}
            (RX, Y) => Ok((RX, Z, Y)),    //ThreeDPoints{x:-1,y:3,z:2}
            (RX, RY) => Ok((RX, RZ, RY)), //ThreeDPoints{x:-1,y:-3,z:-2}

            (Y, X) => Ok((Z, X, Y)),      //ThreeDPoints{x:2,y:3,z:1}
            (Y, RX) => Ok((RZ, X, RY)),   //ThreeDPoints{x:2,y:-3,z:-1}
            (Y, Z) => Ok((RY, X, Z)),     //ThreeDPoints{x:2,y:-1,z:3}
            (Y, RZ) => Ok((Y, X, RZ)),    //ThreeDPoints{x:2,y:1,z:-3}
            (RY, X) => Ok((Z, RX, RY)),   //ThreeDPoints{x:-2,y:-3,z:1}
            (RY, RX) => Ok((RZ, RX, Y)),  //ThreeDPoints{x:-2,y:3,z:-1}
            (RY, Z) => Ok((Y, RX, Z)),    //ThreeDPoints{x:-2,y:1,z:3}
            (RY, RZ) => Ok((RY, RX, RZ)), //ThreeDPoints{x:-2,y:-1,z:-3}

            (Z, X) => Ok((Z, RY, X)),     //ThreeDPoints{x:3,y:-2,z:1}
            (Z, RX) => Ok((RZ, Y, X)),    //ThreeDPoints{x:3,y:2,z:-1}
            (Z, Y) => Ok((Y, Z, X)),      //ThreeDPoints{x:3,y:1,z:2}
            (Z, RY) => Ok((RY, RZ, X)),   //ThreeDPoints{x:3,y:-1,z:-2}
            (RZ, X) => Ok((Z, Y, RX)),    //ThreeDPoints{x:-3,y:2,z:1}
            (RZ, RX) => Ok((RZ, RY, RX)), //ThreeDPoints{x:-3,y:-2,z:-1}
            (RZ, Y) => Ok((RY, Z, RX)),   //ThreeDPoints{x:-3,y:-1,z:2}
            (RZ, RY) => Ok((Y, RZ, RX)),  //ThreeDPoints{x:-3,y:1,z:-2}

            _ => Err(anyhow!(
                "incompatible facing {:?} and up {:?}",
                self.facing,
                self.up
            )),
        }
    }

    fn get_coordinates(&self, point: &ThreeDPoints) -> ThreeDPoints {
        let (ax, ay, az) = self.get_inverse_axes().unwrap();
        ThreeDPoints {
            x: ax.get_unary_coordinate(point),
            y: ay.get_unary_coordinate(point),
            z: az.get_unary_coordinate(point),
        }
    }
}

#[derive(Debug)]
struct Scanner {
    points: Vec<ThreeDPoints>,
}
impl Scanner {
    fn get_matching_scandir(&self, rhs: &Scanner) -> Option<(ScanDir, Vect)> {
        let my_points: HashSet<ThreeDPoints> = self.points.iter().copied().collect();
        for sd in ScanDir::get_all_scandirs() {
            // let's rotate 2n scanner
            let rhs_points: Vec<_> = rhs.points.iter().map(|p| sd.get_coordinates(p)).collect();

            // then try all candidate translations
            for i in 0..rhs_points.len() {
                for j in 0..self.points.len() {
                    let translation = self.points[j] - rhs_points[i];
                    let matches_counts = rhs_points
                        .iter()
                        .filter(|p| my_points.contains(&(**p + translation)))
                        .count();
                    if matches_counts >= 12 {
                        return Some((*sd, translation));
                    }
                }
            }
        }
        None
    }
}

fn read_scanners(input: &str) -> Vec<Scanner> {
    let lines = input.lines();
    let delimiters = lines.clone().enumerate().filter_map(|(i, l)| {
        if l.starts_with("--- scanner ") {
            Some(i)
        } else {
            None
        }
    });

    delimiters
        .map(|i| {
            let mut data = lines.clone();
            // this will  fast forward data to the right position
            let _scanner = data.nth(i).unwrap();
            Scanner {
                points: data
                    .take_while(|l| !l.starts_with("--- scanner "))
                    .filter_map(|l| l.parse::<ThreeDPoints>().ok())
                    .collect(),
            }
        })
        .collect()
}

fn tune_scanners(scanners: &[Scanner]) -> (Vec<Scanner>, Vec<ThreeDPoints>) {
    let mut converted_idx = vec![0];
    let mut converted: Vec<Scanner> = vec![Scanner {
        points: scanners[0].points.clone(),
    }];
    let mut scanners_coordinates = vec![ThreeDPoints { x: 0, y: 0, z: 0 }];

    let mut already_checked_against = 0;
    for _ in 0..scanners.len() {
        let checked_against = converted.len();
        for (j, scanner) in scanners.iter().enumerate() {
            if !converted_idx.contains(&j) {
                // eprint!(".");
                if let Some((sd, trans)) = converted.iter().enumerate().find_map(|(u, conv)| {
                    if u >= already_checked_against {
                        conv.get_matching_scandir(scanner)
                    } else {
                        None
                    }
                }) {
                    // eprintln!("#{:?}", sd);
                    converted_idx.push(j);
                    scanners_coordinates.push(ThreeDPoints { x: 0, y: 0, z: 0 } + trans);
                    converted.push(Scanner {
                        points: scanners[j]
                            .points
                            .iter()
                            .map(|p| sd.get_coordinates(p) + trans)
                            .collect(),
                    });
                }
            }
        }
        already_checked_against = checked_against;
    }
    if converted.len() < scanners.len() {
        panic!(
            "could not place {:?} scanners relative to others",
            (0..scanners.len())
                .filter(|i| !converted_idx.contains(i))
                .collect_vec()
        );
    }
    (converted, scanners_coordinates)
}

fn max_manhattan_distance(scan_coords: &[ThreeDPoints]) -> usize {
    scan_coords
        .iter()
        .flat_map(|p1| {
            scan_coords
                .iter()
                .map(|p2| (p2.x - p1.x).abs() + (p2.y - p1.y).abs() + (p2.z - p1.z).abs())
        })
        .max()
        .unwrap() as usize
}

pub fn display_breacons_and_scanners() {
    let input = include_str!("../ressources/day19_scanners.txt");
    let scanners = read_scanners(input);

    let (scanners, coordinates) = tune_scanners(&scanners);

    let beacons: HashSet<_> = scanners.iter().flat_map(|s| s.points.iter()).collect();

    println!("there are {} different beacons", beacons.len());
    println!(
        "max manhattan distance between scanners is {}",
        max_manhattan_distance(&coordinates)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"
    }

    #[test]
    fn scanners_can_be_decoded() {
        let scanners = read_scanners(get_input());
        assert_eq!(5, scanners.len());
        assert_eq!(26, scanners[4].points.len())
    }

    #[test]
    fn scandirsrs_can_be_rotated() {
        let scanners = read_scanners(get_input());
        let refp = ThreeDPoints { x: 1, y: 2, z: 3 };

        assert_eq!(
            refp,
            ScanDir { facing: Y, up: RX }.get_coordinates(&ThreeDPoints { x: 2, y: -3, z: -1 })
        );
    }

    #[test]
    fn scanners_can_be_positioned_maybe() {
        let scanners = read_scanners(get_input());
        assert!(scanners[0].get_matching_scandir(&scanners[1]).is_some());
        assert!(scanners[0].get_matching_scandir(&scanners[2]).is_none());

        let (scanners, coords) = tune_scanners(&read_scanners(get_input()));
        let beacons: HashSet<_> = scanners.iter().flat_map(|s| s.points.iter()).collect();

        assert_eq!(79, beacons.len());
        assert_eq!(3621, max_manhattan_distance(&coords));
    }
}
