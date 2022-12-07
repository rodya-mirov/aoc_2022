use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

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
