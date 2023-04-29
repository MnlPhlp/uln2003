//! 28BYJ-48/ULN2003 Driver
//!
//! Platform-agnostic driver API for the 28BYJ-48 stepper motor used with the ULN2003 driver. Can be
//! used on any platform for which implementations of the required
//! [embedded-hal] traits are available.
#![no_std]
#![deny(missing_docs)]

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::PinState::{self, High, Low};

/// different positions of the motor.
/// Depending on the state different pins have to be high
/// |wire | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
/// | --- | - | - | - | - | - | - | - | - | - |
/// |  1  |   |   |   |   |   |   | x | x | x |
/// |  2  |   |   |   |   | x | x | x |   |   |
/// |  3  |   |   | x | x | x |   |   |   |   |
/// |  4  |   | x | x |   |   |   |   |   | x |
#[derive(Copy, Clone, Debug)]
enum State {
    State0,
    State1,
    State2,
    State3,
    State4,
    State5,
    State6,
    State7,
    State8,
}

fn get_pin_states(s: State) -> [PinState; 4] {
    match s {
        State::State0 => [Low, Low, Low, Low],
        State::State1 => [Low, Low, Low, High],
        State::State2 => [Low, Low, High, High],
        State::State3 => [Low, Low, High, Low],
        State::State4 => [Low, High, High, Low],
        State::State5 => [Low, High, Low, Low],
        State::State6 => [High, High, Low, Low],
        State::State7 => [High, Low, Low, Low],
        State::State8 => [High, Low, Low, High],
    }
}

fn get_next_state(s: State) -> State {
    match s {
        State::State0 => State::State1,
        State::State1 => State::State2,
        State::State2 => State::State3,
        State::State3 => State::State4,
        State::State4 => State::State5,
        State::State5 => State::State6,
        State::State6 => State::State7,
        State::State7 => State::State8,
        State::State8 => State::State1,
    }
}

fn get_prev_state(s: State) -> State {
    match s {
        State::State0 => State::State8,
        State::State1 => State::State8,
        State::State2 => State::State1,
        State::State3 => State::State2,
        State::State4 => State::State3,
        State::State5 => State::State4,
        State::State6 => State::State5,
        State::State7 => State::State6,
        State::State8 => State::State7,
    }
}

/// Struct representing a Stepper motor with the 4 driver pins
pub struct ULN2003<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin> {
    in1: P1,
    in2: P2,
    in3: P3,
    in4: P4,
    state: State,
    dir: Direction,
}

impl<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin> ULN2003<P1, P2, P3, P4> {
    /// Create a new StepperMotor from the 4 pins connected to te uln2003 driver
    pub fn new(in1: P1, in2: P2, in3: P3, in4: P4) -> Self {
        Self {
            in1,
            in2,
            in3,
            in4,
            state: State::State0,
            dir: Direction::Normal,
        }
    }
}

impl<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin> StepperMotor
    for ULN2003<P1, P2, P3, P4>
{
    fn step(&mut self) {
        let states = get_pin_states(self.state);
        self.in1.set_state(states[0]);
        self.in2.set_state(states[1]);
        self.in3.set_state(states[2]);
        self.in4.set_state(states[3]);
        match self.dir {
            Direction::Normal => self.state = get_next_state(self.state),
            Direction::Reverse => self.state = get_prev_state(self.state),
        }
    }

    fn set_direction(&mut self, dir: Direction) {
        self.dir = dir;
    }
}

/// trait to prevent having to pass around the struct with all the generic arguments
pub trait StepperMotor {
    /// Do a single step
    fn step(&mut self);
    /// Set the stepping direction
    fn set_direction(&mut self, dir: Direction);
}

/// Direction the motor turns in. Just reverses the order of the internal states.
pub enum Direction {
    /// Default direction
    Normal,
    /// Reversed direction
    Reverse,
}
