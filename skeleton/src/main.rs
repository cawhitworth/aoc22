mod error;
use error::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input")?;

    let mut total_score = 0;
    {
        let reader = BufReader::new(input);
        for line in reader.lines() {
            let line_str = line?;
        }
    }

    println!("Score: {}", total_score);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert!(true);
    }
}
