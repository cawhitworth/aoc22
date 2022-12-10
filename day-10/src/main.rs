use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Instr {
    Noop,
    Addx(i32),
}

fn parse_instrs(line: &str) -> anyhow::Result<Instr> {
    let (instr, rest) = line.split_at(4);
    match instr {
        "noop" => Ok(Instr::Noop),
        "addx" => Ok(Instr::Addx(rest.trim().parse()?)),
        _ => Err(anyhow::anyhow!("Invalid instr: {}", line)),
    }
}

struct Cpu {
    tick: usize,
    x: i32,
    crt: [char; 260],
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            tick: 1,
            x: 1,
            crt: [' '; 260],
        }
    }

    fn update_screen(&mut self) {
        let crt_tick = self.tick - 1;
        let beam = crt_tick as i32 % 40;
        self.crt[crt_tick] = if beam > self.x - 2 && beam < self.x + 2 {
            '#'
        } else {
            '.'
        };
    }

    fn check_signal(&mut self) -> Option<i32> {
        if (self.tick as i32 - 20) % 40 == 0 {
            let signal_strength = self.tick as i32 * self.x;
            Some(signal_strength)
        } else {
            None
        }
    }

    fn run<I>(&mut self, instrs: I) -> anyhow::Result<(i32, i32)>
    where
        I: Iterator<Item = Instr>,
    {
        let mut total_signal_strength = 0;
        for instr in instrs {
            match instr {
                Instr::Noop => {
                    self.update_screen();
                    self.tick += 1;
                    if let Some(signal_strength) = self.check_signal() {
                        total_signal_strength += signal_strength;
                    }
                }
                Instr::Addx(v) => {
                    self.update_screen();
                    self.tick += 1;
                    if let Some(signal_strength) = self.check_signal() {
                        total_signal_strength += signal_strength;
                    }
                    self.update_screen();
                    self.tick += 1;
                    self.x += v;
                    if let Some(signal_strength) = self.check_signal() {
                        total_signal_strength += signal_strength;
                    }
                }
            }
        }
        Ok((self.x, total_signal_strength))
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;
    let instructions = input
        .lines()
        .map(|l| parse_instrs(l).expect("Failed to parse"));

    let mut cpu = Cpu::new();

    let (_, signal_strength) = cpu.run(instructions)?;

    println!("Signal strength: {}", signal_strength);

    let lines = cpu.crt.iter().collect::<String>();
    for i in 0..6 {
        let line = &lines[i * 40..(i + 1) * 40];
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut lines = TEST_DATA.lines();
        let instrs = parse_instrs(lines.next().unwrap())?;
        assert_eq!(instrs, Instr::Addx(15));

        Ok(())
    }

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let lines = TEST_DATA
            .lines()
            .map(|l| parse_instrs(l).expect("Failed to parse"));

        let mut cpu = Cpu::new();

        let (_, signal_strength) = cpu.run(lines)?;

        assert_eq!(signal_strength, 13140);

        Ok(())
    }
}
