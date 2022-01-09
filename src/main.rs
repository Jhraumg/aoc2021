mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::time::{Duration, Instant};

use colored::*;
use itertools::Itertools;

struct Timer {
    start: Instant,
    last: Instant,
}
impl Timer {
    fn new() -> Self {
        let instant = Instant::now();
        Self {
            last: instant,
            start: instant,
        }
    }

    fn click(&mut self) {
        static ONE_SECOND: Duration = Duration::from_secs(1);
        let elapsed = self.last.elapsed();
        if elapsed > ONE_SECOND {
            println!(
                "{}\n",
                format!("*** {} s ***", elapsed.as_secs()).truecolor(255, 128, 0)
            );
        } else {
            println!(
                "{}\n",
                format!("*** {} ms ***", elapsed.as_millis()).truecolor(0, 255, 0)
            );
        }

        self.last = Instant::now();
    }

    fn display_total(&self) {
        println!(
            "{}",
            format!("*** TOTAL : {} s ***", self.start.elapsed().as_secs()).truecolor(0, 255, 0)
        );
    }
}

fn main() {
    let mut timer = Timer::new();
    println!("{}", "********".truecolor(0, 255, 0));
    day1::print_depth_incrs();
    timer.click();

    day2::print_position();
    timer.click();

    day3::print_power();
    timer.click();

    day4::display_bingo();
    timer.click();

    day5::print_hydrothermals();
    timer.click();

    day6::print_lanternfishes_counts();
    timer.click();

    day7::print_crab_alignment();
    timer.click();

    day8::display_digits();
    timer.click();

    day9::display_smoke_risks();
    timer.click();

    day10::print_syntax_check();
    timer.click();

    day11::display_octopuses_flash_count();
    timer.click();

    day12::display_pathes();
    timer.click();

    day13::print_origami_details();
    timer.click();

    day14::display_polymer();
    timer.click();

    day15::display_safest_path();
    timer.click();

    day16::print_bits();
    timer.click();

    day17::display_trajectory();
    timer.click();

    day18::display_additions();
    timer.click();

    day19::display_breacons_and_scanners();
    timer.click();

    day20::display_enhanced_img();
    timer.click();

    day21::display_dirac_dice_play();
    timer.click();

    day22::display_reactor_reboot();
    timer.click();

    day23::organize_amphipods();
    timer.click();

    day24::print_larget_serial_accepted_by_monad();
    timer.click();

    day25::find_spot_on_sea_floor();
    timer.click();
    timer.display_total();
    println!(
        "{}",
        vec!["*"; 26]
            .iter()
            .enumerate()
            .map(|(i, s)| s.truecolor(i as u8 * 10, 255 - i as u8 * 10, i as u8 * 10))
            .join("")
    );
}
