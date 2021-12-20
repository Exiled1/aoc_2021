use std::num::ParseIntError;
use std::result::Result as StdResult;
use std::{
    error,
    fs::File,
    io::{prelude::*, BufReader},
    result,
};
mod day1;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let file = File::open("./src/sonar.txt")?;
    let reader = BufReader::new(file);
    let sea_floor: Vec<i32> = reader
        .lines()
        .collect::<StdResult<Vec<_>, std::io::Error>>()?
        .iter()
        .map(|f| f.parse::<i32>())
        .collect::<StdResult<Vec<_>, ParseIntError>>()?;

    // println!("{:?}", sonar_sweep(&sea_floor));
    println!("{:?}", day1::triple_sonar(&sea_floor));

    Ok(())
}

