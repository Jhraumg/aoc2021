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

use colored::*;
use itertools::Itertools;

fn main() {
    println!("{}", "****".truecolor(0, 255, 0));
    day1::print_depth_incrs();
    println!("{}", "****".truecolor(0, 255, 0));
    day2::print_position();
    println!("{}", "****".truecolor(0, 255, 0));
    day3::print_power();
    println!("{}", "****".truecolor(0, 255, 0));
    day4::display_bingo();
    println!("{}", "****".truecolor(0, 255, 0));
    day5::print_hydrothermals();
    println!("{}", "****".truecolor(0, 255, 0));
    day6::print_lanternfishes_counts();
    println!("{}", "****".truecolor(0, 255, 0));
    day7::print_crab_alignment();
    println!("{}", "****".truecolor(0, 255, 0));
    day8::display_digits();
    println!("{}", "****".truecolor(0, 255, 0));
    day9::display_smoke_risks();
    println!("{}", "****".truecolor(0, 255, 0));
    day10::print_syntax_check();
    println!("{}", "****".truecolor(0, 255, 0));
    day11::display_octopuses_flash_count();
    println!("{}", "****".truecolor(0, 255, 0));
    day12::display_pathes();
    println!("{}", "****".truecolor(0, 255, 0));
    day13::print_origami_details();
    println!("{}", "****".truecolor(0, 255, 0));
    day14::display_polymer();
    println!("{}", "****".truecolor(0, 255, 0));
    day15::display_safest_path();
    println!("{}", "****".truecolor(0, 255, 0));
    day16::print_bits();
    println!("{}", "****".truecolor(0, 255, 0));
    day17::display_trajectory();
    println!("{}", "****".truecolor(0, 255, 0));
    day18::display_additions();
    println!("{}", "****".truecolor(0, 255, 0));
    day19::display_breacons_and_scanners();
    println!("{}", "****".truecolor(0, 255, 0));
    day20::display_enhanced_img();
    println!("{}", "****".truecolor(0, 255, 0));
    day21::display_dirac_dice_play();
    println!("{}", "****".truecolor(0, 255, 0));
    day22::display_reactor_reboot();
    println!("{}", "****".truecolor(0, 255, 0));
    day23::organize_amphipods();
    println!("{}", "****".truecolor(0, 255, 0));
    day24::print_larget_serial_accepted_by_monad();
    println!("{}", "****".truecolor(0, 255, 0));
    day25::find_spot_on_sea_floor();
    println!(
        "{}",
        vec!["*"; 26]
            .iter()
            .enumerate()
            .map(|(i, s)| s.truecolor(i as u8 * 10, 255 - i as u8 * 10, i as u8 * 10))
            .join("")
    );
}
