#[macro_use]
extern crate clap;

pub mod util;


mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use clap::App;

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: u32) {
    match day {
        1 => day_1::run_day_1(),
        2 => day_2::run_day_2(),
        3 => day_3::run_day_3(),
        4 => day_4::run_day_4(),
        5 => day_5::run_day_5(),
        6 => day_6::run_day_6(),
        7 => day_7::run_day_7(),
        8 => day_8::run_day_8(),
        9 => day_9::run_day_9(),
        10 => day_10::run_day_10(),
        11 => day_11::run_day_11(),
        12 => day_12::run_day_12(),
        13 => day_13::run_day_13(),
        14 => day_14::run_day_14(),
        15 => day_15::run_day_15(),
        16 => day_16::run_day_16(),
        17 => day_17::run_day_17(),
        18 => day_18::run_day_18(),
        19 => day_19::run_day_19(),
        20 => day_20::run_day_20(),
        21 => day_21::run_day_21(),
        22 => day_22::run_day_22(),
        23 => day_23::run_day_23(),
        24 => day_24::run_day_24(),
        25 => day_25::run_day_25(),
        _ => panic!("Can't run this day!!!!!!"),
    }
}

fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    let day: u32 = value_t!(matches.value_of("day"), u32).unwrap();

    print_seperator();
    println!("Running Day: {}", day);
    print_seperator();
    
    run_day(day);
}
