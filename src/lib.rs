use rand::{thread_rng, Rng};
use std::{
    fmt::Display,
    ops::{Add, Rem, Sub},
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
    wrap: bool,
    disp: [char; 2],
}
impl Game {
    pub fn new(rule: u8, width: usize, wrap: bool, disp: [char; 2]) -> Game {
        Game {
            state: vec![false; width],
            size: width,
            rule,
            wrap,
            disp,
        }
    }

    pub fn randomize(&mut self) {
        thread_rng().fill(&mut self.state[..]);
    }

    pub fn set_state(&mut self, state: &[bool]) {
        if state.len() > self.size {
            panic!(
                "Given state is too large ({}) for game width: {}",
                state.len(),
                self.size
            );
        }
        self.state = vec![false; self.size];
        for (i, n) in state.into_iter().enumerate() {
            self.state[i] = *n;
        }
    }

    pub fn set_only(&mut self, n: usize) {
        if n > self.size {
            panic!(
                    "Given index is too large ({}) for game width: {}", n, self.size
            );
        }
        self.state = vec![false; self.size];
        self.state[n] = true;
    }

    pub fn step_and_update(&mut self) {
        self.state = self.step();
    }

    pub fn width(&self) -> usize {
        self.size
    }

    fn neighbors_wrapping(&self, n: usize) -> u8 {
        ((self.state[n.sub_mod(1, self.size)] as u8) << 2)
            ^ ((self.state[n] as u8) << 1)
            ^ (self.state[n.add_mod(1, self.size)] as u8)
    }

    fn neighbors_not_wrapping(&self, n: usize) -> u8 {
        (if n > 0 {self.state[n - 1] as u8} else {0} << 2)
            ^ ((self.state[n] as u8) << 1)
            ^ (if n < self.size - 1 {self.state[n + 1] as u8} else {0})
    }

    fn step(&self) -> Vec<bool> {
        let mut next_step = vec![false; self.size];

        if self.wrap {
            for i in 0..self.state.len() {
                next_step[i] = (1 << self.neighbors_wrapping(i)) & self.rule != 0
            }
        } else {
            for i in 0..self.state.len() {
                next_step[i] = (1 << self.neighbors_not_wrapping(i)) & self.rule != 0
            }
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
