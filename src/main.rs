extern crate clap;
extern crate crossterm;
extern crate rand;
mod lib;

use crate::lib::Game;
use clap::{error::ErrorKind, ArgGroup, Parser};
use crossterm::{cursor::MoveUp, ExecutableCommand};
use std::{
    io::{stdin, stdout},
    num::IntErrorKind,
};

#[derive(Parser, Debug)]
#[command(group(
            ArgGroup::new("init")
                .args(["random", "custom"]),
        ))]
#[command(group(
            ArgGroup::new("chars")
                .args(["display", "block"]),
        ))]
struct Cli {
    rule: u8,
    #[arg(short, long, default_value_t = 100)]
    width: usize,
    #[arg(short, long)]
    iterations: Option<usize>,
    #[arg(short, long)]
    display: Option<String>,
    #[arg(long)]
    block: bool,
    #[arg(short, long)]
    random: bool,
    custom: Option<String>,
}

const DEFAULT_CHARS: [char; 2] = ['X', '.'];
const BLOCK_CHARS: [char; 2] = ['█', ' '];
fn main() {
    let cli: Cli = Cli::parse();

    let display = if cli.block {
        BLOCK_CHARS
    } else if let Some(disp) = cli.display {
        if disp.len() != 2 {
            clap::Error::raw(
                ErrorKind::InvalidValue,
                "Input to --display must be a string of two characters i.e. \"X.\" or \"█ \"\n",
            )
            .exit()
        }
        let mut c = disp.chars();
        [c.next().unwrap(), c.next().unwrap()]
    } else {
        DEFAULT_CHARS
    };
    let mut game = Game::new(cli.rule, cli.width, display);
    if cli.random {
        game.randomize();
    } else if let Some(init) = cli.custom {
        let mut vec = Vec::<bool>::with_capacity(game.width());
        for c in init.chars() {
            vec.push(match c {
                ' ' | '.' | '0' => false,
                'x' | 'X' | '1' | '█' => true,
                _ => clap::Error::raw(
                    ErrorKind::InvalidValue,
                    "Custom input must only be [' ', '.', '0'] (dead) or ['x', 'X', '1'] (alive)\n",
                )
                .exit(),
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
