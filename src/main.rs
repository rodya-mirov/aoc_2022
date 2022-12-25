use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
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
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

fn main() -> Result<(), ()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: [run command] day");
        println!("  Example: cargo run --release -- 12b");
        return Err(());
    }

    let start = std::time::SystemTime::now();

    let answer = match args.nth(1).unwrap().as_str() {
        "1a" => day_01::a(),
        "1b" => day_01::b(),

        "2a" => day_02::a(),
        "2b" => day_02::b(),

        "3a" => day_03::a(),
        "3b" => day_03::b(),

        "4a" => day_04::a(),
        "4b" => day_04::b(),

        "5a" => day_05::a(),
        "5b" => day_05::b(),

        "6a" => day_06::a(),
        "6b" => day_06::b(),

        "7a" => day_07::a(),
        "7b" => day_07::b(),

        "8a" => day_08::a(),
        "8b" => day_08::b(),

        "9a" => day_09::a(),
        "9b" => day_09::b(),

        "10a" => day_10::a(),
        "10b" => day_10::b(),

        "11a" => day_11::a(),
        "11b" => day_11::b(),

        "12a" => day_12::a(),
        "12b" => day_12::b(),

        "13a" => day_13::a(),
        "13b" => day_13::b(),

        "14a" => day_14::a(),
        "14b" => day_14::b(),

        "15a" => day_15::a(),
        "15b" => day_15::b(),

        "16a" => day_16::a(),
        "16b" => day_16::b(),

        "17a" => day_17::a(),
        "17b" => day_17::b(),

        "18a" => day_18::a(),
        "18b" => day_18::b(),

        "19a" => day_19::a(),
        "19b" => day_19::b(),

        "20a" => day_20::a(),
        "20b" => day_20::b(),

        "21a" => day_21::a(),
        "21b" => day_21::b(),

        "22a" => day_22::a(),
        "22b" => day_22::b(),

        "23a" => day_23::a(),
        "23b" => day_23::b(),

        "24a" => day_24::a(),
        "24b" => day_24::b(),

        "25a" => day_25::a(),
        "25b" => day_25::b(),

        other => {
            println!("Unknown day variant {:?}", other);
            return Err(());
        }
    };

    let elapsed = start.elapsed().unwrap();

    println!("Answer: {}", answer);
    println!("Elapsed: {:.5} seconds", elapsed.as_secs_f32());

    Ok(())
}
