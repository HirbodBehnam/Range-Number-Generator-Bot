use lazy_static::lazy_static;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::RwLock;
use teloxide::prelude::*;

/// Error message to send to users when they message is not number
const ERROR_MESSAGE: &str = "لطفا شماره ی دانشجویی خود را وارد کنید.";
/// The title of the result which users receive when they enter a number
const TOP_TEXT: &str = "این سوالات را حل کنید:";

/// Represents a range of numbers
struct RangeStruct {
    min: i64,
    max: i64,
}

/// Represents an ranges of functions
struct RangesStruct {
    ranges: Vec<RangeStruct>,
}

lazy_static! {
    static ref RANGES: RwLock<RangesStruct> = RwLock::new(RangesStruct { ranges: vec![] });
}

#[tokio::main]
async fn main() {
    read_config();
    run().await;
}

/// Reads the config file of number ranges
fn read_config() {
    let lines = read_lines("ranges.txt").expect("cannot read the config file");
    for line in lines {
        if let Ok(range) = line {
            let split_range: Vec<&str> = range.split(" ").collect();
            RANGES.write().unwrap().ranges.push(RangeStruct {
                min: split_range[0].parse::<i64>().expect("cannot parse the min"),
                max: split_range[1].parse::<i64>().expect("cannot parse the max"),
            })
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Runs the bot
async fn run() {
    teloxide::enable_logging!();
    let bot = Bot::from_env().auto_send();
    teloxide::repl(bot, |message| async move {
        // Drop messages without text
        if message.update.text().is_none() {
            message.answer(ERROR_MESSAGE).await?;
            return respond(());
        }
        // Drop messages with non-number text
        let student_number = message.update.text().unwrap().parse::<u64>();
        if student_number.is_err() {
            message.answer(ERROR_MESSAGE).await?;
            return respond(());
        }
        // Get the number
        message
            .answer(derive_numbers(student_number.unwrap()).await)
            .await?;
        return respond(());
    })
    .await;
}

/// Derives numbers in range of RANGES from the student number
async fn derive_numbers(student_number: u64) -> String {
    let mut rng = ChaCha20Rng::seed_from_u64(student_number);
    let mut result = String::from(TOP_TEXT);
    for range in RANGES.read().unwrap().ranges.iter() {
        let i = rng.gen_range(range.min..range.max);
        result.push('\n');
        result.push_str(i.to_string().as_str());
    }
    result
}
