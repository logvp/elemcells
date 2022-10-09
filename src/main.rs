extern crate clap;
extern crate crossterm;
extern crate rand;
use clap::{ArgGroup, Parser};
use crossterm::{cursor::MoveUp, ExecutableCommand};
use rand::{thread_rng, Rng};
use std::{
    fmt::Display,
    io::{stdin, stdout},
    num::{IntErrorKind, ParseIntError},
    ops::{Add, Index, Rem, Sub},
};

trait ModularArith<Rhs = Self> {
    type Output;
    fn add_mod(self, rhs: Rhs, modulo: Self) -> <Self as ModularArith<Rhs>>::Output;
    fn sub_mod(self, rhs: Rhs, modulo: Self) -> <Self as ModularArith<Rhs>>::Output;
    fn modulus(self, rhs: Rhs) -> <Self as ModularArith<Rhs>>::Output;
}
impl<T> ModularArith for T
where
    T: Rem<Self, Output = Self> + Add<Self, Output = Self> + Sub<Self, Output = Self> + Copy,
{
    type Output = Self;
    fn add_mod(self, rhs: Self, modulo: Self) -> Self {
        (self.modulus(modulo) + rhs.modulus(modulo)).modulus(modulo)
    }
    fn sub_mod(self, rhs: Self, modulo: Self) -> Self {
        (self.modulus(modulo) + modulo - rhs.modulus(modulo)).modulus(modulo)
    }
    fn modulus(self, rhs: Self) -> Self {
        ((self % rhs) + rhs) % rhs
    }
}
/*
impl ModularArith for usize {
    type Output = Self;
    fn add_mod(self, rhs: Self, modulo: Self) -> Self {
        (self.rem_euclid(modulo) + rhs.rem_euclid(modulo)).rem_euclid(modulo)
    }
    fn sub_mod(self, rhs: Self, modulo: Self) -> Self {
        (self.rem_euclid(modulo) - rhs.rem_euclid(modulo)).rem_euclid(modulo)
    }
    fn modulus(self, rhs: Self) -> Self {
        self.rem_euclid(rhs)
    }
}
*/

struct Game {
    state: Vec<bool>,
    size: usize,
    rule: u8,
}
impl Index<usize> for Game {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index]
    }
}
impl Game {
    fn new(rule: u8, width: usize) -> Game {
        Game {
            state: vec![false; width],
            size: width,
            rule,
        }
    }

    fn randomize(&mut self) {
        thread_rng().fill(&mut self.state[..]);
    }

    fn set_state(&mut self, state: &[bool]) {
        if state.len() > self.size {
            panic!(
                "Given state is too large ({}) for game dimension: {}",
                state.len(),
                self.size
            );
        }
        self.state = vec![false; self.size];
        for (i, n) in state.into_iter().enumerate() {
            self.state[i] = *n;
        }
    }

    fn step_and_update(&mut self) {
        self.state = self.step();
    }

    fn neighbors(&self, n: usize) -> u8 {
        ((self.state[n.sub_mod(1, self.size)] as u8) << 2)
            ^ ((self.state[n] as u8) << 1)
            ^ (self.state[n.add_mod(1, self.size)] as u8)
    }

    fn step(&self) -> Vec<bool> {
        let mut next_step = vec![false; self.size];

        for i in 0..self.state.len() {
            next_step[i] = (1 << self.neighbors(i)) & self.rule != 0
        }

        next_step
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.state.iter() {
            write!(f, "{}", if *b { "X" } else { "." })?
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(group(
            ArgGroup::new("init")
                .args(["random", "custom"]),
        ))]
struct Cli {
    rule: u8,
    #[arg(short, long, default_value_t = 100)]
    width: usize,
    #[arg(short, long)]
    iterations: Option<usize>,
    #[arg(short, long)]
    random: bool,
    custom: Option<String>,
}

fn main() {
    let cli: Cli = Cli::parse();
    println!("{:?}", cli);
    let mut game = Game::new(cli.rule, cli.width);
    if cli.random {
        game.randomize();
    } else if let Some(init) = cli.custom {
        let mut vec = Vec::<bool>::with_capacity(game.size);
        for c in init.chars() {
            vec.push(match c {
                ' ' | '.' | '0' => false,
                'x' | 'X' | '1' => true,
                _ => panic!(
                    "Custom input must only be [' ', '.', '0'] (dead) or ['x', 'X', '1'] (alive)"
                ),
            })
        }
        game.set_state(&vec);
    } else {
        // default state is X......(...)
        game.set_state(&[true]);
    }
    if let Some(itt) = cli.iterations {
        for _ in 0..itt {
            println!("{}", game);
            game.step_and_update();
        }
    } else {
        enum Control {
            Quit,
            Nothing,
            Continue,
            Iterate(u32),
        }
        let mut stdout = stdout();
        let stdin = stdin();
        let mut buf = String::new();
        let mut get_input = || -> Control {
            buf.clear();
            stdin
                .read_line(&mut buf)
                .expect("Failed to read line from stdin");
            buf = buf.trim().to_string();
            if buf.eq_ignore_ascii_case("q") || buf.eq_ignore_ascii_case("quit") {
                return Control::Quit;
            }
            match buf.parse::<u32>() {
                Ok(num) => Control::Iterate(num),
                Err(error) => match *error.kind() {
                    IntErrorKind::Empty => Control::Continue,
                    IntErrorKind::InvalidDigit => {
                        println!("Expected empty or number");
                        Control::Nothing
                    }
                    IntErrorKind::NegOverflow => {
                        println!("Unfortunately the simulation can't go backwards");
                        Control::Nothing
                    }
                    IntErrorKind::PosOverflow => {
                        println!("Too large");
                        Control::Nothing
                    }
                    IntErrorKind::Zero => panic!("Unreachable"),
                    _ => panic!("Unreachable"),
                },
            }
        };
        let mut step = || {
            println!("{}", game);
            game.step_and_update();
        };
        step();
        loop {
            match get_input() {
                Control::Continue => {
                    stdout.execute(MoveUp(1)).unwrap();
                    step();
                }
                Control::Iterate(n) => {
                    for _ in 0..n {
                        step();
                    }
                }
                Control::Nothing => {}
                Control::Quit => return,
            }
        }
    }
}
