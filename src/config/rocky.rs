#[derive(Serialize, Deserialize, Debug)]
pub struct Rocky {
  /// The chance of a rocky planet having a moon
  #[serde(default = "Rocky::default_moon")]
  pub moon: f32,

  /// The chance of a rocky planet having a belt
  #[serde(default = "Rocky::default_belt")]
  pub belt: f32,
}

impl Rocky {
  fn default_belt() -> f32 {
    0.2
  }
  fn default_moon() -> f32 {
    0.4
  }
}

impl Default for Rocky {
  fn default() -> Self {
    Rocky {
      belt: Self::default_belt(),
      moon: Self::default_moon(),
    }
  }
}
