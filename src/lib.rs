// TODO Better Documentation.
extern crate rand;
use rand::Rng;
use std::ops::{Add, Div, Mul, Sub};
use std::convert::Into;

/// All dice must implement this trait on at least 1 number type.
pub trait Dice<T>: Add<T> + Sub<T> + Div<T> + Mul<T> + Into<T> {
  /// NdX, where X is the type of Dice, and N is the parameter.
  fn n(self, n: T) -> T;
  /// From NdX, keep the lowest `kl`.
  fn nkl(self, n: T, kl: T) -> T;
  /// From NdX, keep the highest `kh`.
  fn nkh(self, n: T, kh: T) -> T;
  /// From NdX, drop the lowest `dl`.
  fn ndl(self, n: T, dl: T) -> T;
  /// From NdX, drop the highest `dh`.
  fn ndh(self, n: T, dh: T) -> T;
}

#[macro_use]
mod macros;

dice! {
  /// A 2 sided die, basically for when you want a coin flip.
  D2: 2
}

dice! {
  /// A 4 sided die.
  D4: 4
}

dice! {
  /// A 6 sided die.
  D6: 6
}

dice! {
  /// An 8 sided die.
  D8: 8
}

dice! {
  /// A 10 sided die.
  D10: 10
}

dice! {
  /// A 12 sided die.
  D12: 12
}

dice! {
  /// A 20 sided die.
  D20: 20
}

dice! {
  /// A 100 sided die.
  D100: 100
}
