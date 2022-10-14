extern crate clap;
extern crate crossterm;
extern crate rand;
mod lib;

use crate::lib::Game;
use clap::{error::ErrorKind, ArgGroup, Parser};
use crossterm::{cursor::MoveUp, ExecutableCommand};
use std::{
    io::{stdin, stdout, Write},
    num::IntErrorKind,
};

#[derive(Parser, Debug)]
#[command(
    group(ArgGroup::new("init").args(["random", "custom", "middle"])),
    group(ArgGroup::new("chars").args(["display", "block"])),
    long_about = "Elementary Cellular Automata simulator. A RULE (8 bit number) is required for every simulation as it derives all of the rules of interactions between cells. \
    The program will run in iteractive mode unless the -i argument is set. In interactive mode press enter to advance the simulation one stage, or enter a number to move that \
    many simulation steps"
)]

struct Cli {
    rule: u8,
    #[arg(short, long, default_value_t = 100)]
    width: usize,
    #[arg(
        short,
        long,
        visible_alias = "height",
        help = "If specified simulation will show N many lines and then exit.\nIf unset, simulation will enter interactive mode"
    )]
    iterations: Option<usize>,
    #[arg(
        short,
        long,
        short_alias = 'c',
        help = "A two character string that will be used to visualize the simulation"
    )]
    display: Option<String>,
    #[arg(
        short,
        long,
        help = "Prints cells as solid colors",
        name = "block",
        short_alias = 'b',
        alias = "block"
    )]
    solid: bool,
    #[arg(
        short,
        long,
        help = "Randomizes the initial condition"
    )]
    random: bool,
    #[arg(
        short,
        long,
        help = "Starts with one live cell in the center"
    )]
    middle: bool,
    #[arg(
        help = "A string representing the initial condition for the simulation"
    )]
    custom: Option<String>,
    #[arg(
        short,
        long,
        help = "Treats the edges of the simulation as dead cells"
    )]
    no_wrap: bool,
}

const DEFAULT_CHARS: [char; 2] = ['X', '.'];
const BLOCK_CHARS: [char; 2] = ['█', ' '];
fn main() {
    let cli: Cli = Cli::parse();

    let display = if cli.solid {
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
    let mut game = Game::new(cli.rule, cli.width, !cli.no_wrap, display);
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
    } else if cli.middle {
        game.set_only(game.width() / 2);
    } else {
        // default state is X(...)
        game.set_state(&[true]);
    }
    let mut print_steps = |n: usize| {
        let mut stdout = stdout();
        for _ in 0..n {
            writeln!(stdout, "{}", game).unwrap();
            game.step_and_update();
        }
    };
    if let Some(n) = cli.iterations {
        print_steps(n);
    } else {
        enum Control {
            Quit,
            Nothing,
            Continue,
            Iterate(u32),
        }
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
        print_steps(1);
        loop {
            match get_input() {
                Control::Continue => {
                    stdout().execute(MoveUp(1)).unwrap();
                    print_steps(1);
                }
                Control::Iterate(n) => {
                    print_steps(n as usize); // no real reason why it can't just parse usize. u32 is kinda arbitrary
                }
                Control::Nothing => {}
                Control::Quit => return,
            }
        }
    }
}
