use clap::{App, Arg};
#[macro_use]
extern crate lazy_static;

fn main() {
    let matches = App::new("Advent of Code 2020")
        .author("Matt Bray")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PART")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("DATA")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();
    let file = matches.value_of("file").unwrap();

    match (day, part) {
        ("1", "1") => day_1::part_1(&file),
        ("1", "2") => day_1::part_2(&file),
        ("2", "1") => day_2::part_1(&file),
        ("2", "2") => day_2::part_2(&file),
        ("3", "1") => day_3::part_1(&file),
        ("3", "2") => day_3::part_2(&file),
        _ => println!("Not solved yet"),
    }
}

mod day_1;
mod day_2;
mod day_3;
