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
