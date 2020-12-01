use clap::{App, Arg};

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
        .get_matches();

    match (
        matches.value_of("day").unwrap(),
        matches.value_of("part").unwrap(),
    ) {
        ("1", "1") => day_1::part_1(),
        ("1", "2") => day_1::part_2(),
        _ => println!("Not solved yet"),
    }
}

mod day_1;
