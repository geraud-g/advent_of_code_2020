use std::env;

mod day_01;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => solve_day(args[1].trim().parse().expect("You must enter a number between 1 and 25.")),
        _ => panic!("You must enter a number between 1 and 25.")
    }
}


fn solve_day(day: i32) {
    let fn_day = match day {
        1 => day_01::day_01,
        2 => unimplemented!(),
        3 => unimplemented!(),
        4 => unimplemented!(),
        5 => unimplemented!(),
        6 => unimplemented!(),
        7 => unimplemented!(),
        8 => unimplemented!(),
        9 => unimplemented!(),
        10 => unimplemented!(),
        11 => unimplemented!(),
        12 => unimplemented!(),
        13 => unimplemented!(),
        14 => unimplemented!(),
        15 => unimplemented!(),
        16 => unimplemented!(),
        17 => unimplemented!(),
        18 => unimplemented!(),
        19 => unimplemented!(),
        20 => unimplemented!(),
        21 => unimplemented!(),
        22 => unimplemented!(),
        23 => unimplemented!(),
        24 => unimplemented!(),
        25 => unimplemented!(),
        _ => panic!("{} is not a valid value", day)
    };
    println!("# Processing Day {} :", day);
    fn_day()
}
