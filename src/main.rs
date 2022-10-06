extern crate clap;
extern crate rand;
use clap::Parser;
use rand::{thread_rng, Rng};
use std::{
    fmt::Display,
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

struct Game<const SIZE: usize> {
    state: [bool; SIZE],
    rule: u8,
    // ruleset: HashMap<[bool; 3], bool>, // maybe don't need a map and can just do the calculations?
}
impl<const SIZE: usize> Index<usize> for Game<SIZE> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index % SIZE]
    }
}
impl<const SIZE: usize> Game<SIZE> {
    fn rule(rule: u8) -> Game<SIZE> {
        Game::<SIZE> {
            state: [false; SIZE],
            rule,
        }
    }

    fn randomize(&mut self) {
        thread_rng().fill(&mut self.state[..]);
    }

    fn set_state(&mut self, state: &[bool]) {
        if state.len() > SIZE {
            panic!("too big")
        }
        self.state = [false; SIZE];
        for (i, n) in state.into_iter().enumerate() {
            self.state[i] = *n;
        }
    }

    fn step_and_update(&mut self) {
        self.state = self.step();
    }

    fn neighbors(&self, n: usize) -> u8 {
        ((self.state[n.sub_mod(1, SIZE)] as u8) << 2)
            ^ ((self.state[n] as u8) << 1)
            ^ (self.state[n.add_mod(1, SIZE)] as u8)
    }

    fn step(&self) -> [bool; SIZE] {
        let mut next_step = [false; SIZE];

        for i in 0..self.state.len() {
            next_step[i] = (1 << self.neighbors(i)) & self.rule != 0
        }

        next_step
    }
}
impl<const SIZE: usize> Display for Game<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.state {
            write!(f, "{}", if b { "X" } else { "." })?
        }
        Ok(())
    }
}

fn main() {
    let mut game = Game::<150>::rule(124);
    // game.randomize();
    game.set_state(&[true]);
    println!("Initial State:\n{}", game);
    for _ in 0..100 {
        game.step_and_update();
        println!("{}", game);
    }
}
