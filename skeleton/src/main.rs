use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;

    let lines = input.lines();

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
