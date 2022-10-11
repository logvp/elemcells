use rand::{thread_rng, Rng};
use std::{
    fmt::Display,
    io::{stdin, stdout},
    num::IntErrorKind,
    ops::{Add, Index, Rem, Sub},
};

// Totally overkill but why not
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

pub struct Game {
    state: Vec<bool>,
    size: usize,
    rule: u8,
    disp: [char; 2],
}
impl Index<usize> for Game {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index]
    }
}
impl Game {
    pub fn new(rule: u8, width: usize, disp: [char; 2]) -> Game {
        Game {
            state: vec![false; width],
            size: width,
            rule,
            disp,
        }
    }

    pub fn randomize(&mut self) {
        thread_rng().fill(&mut self.state[..]);
    }

    pub fn set_state(&mut self, state: &[bool]) {
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

    pub fn step_and_update(&mut self) {
        self.state = self.step();
    }

    pub fn width(&self) -> usize {
        self.size
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
            write!(f, "{}", if *b { self.disp[0] } else { self.disp[1] })?
        }
        Ok(())
    }
}
