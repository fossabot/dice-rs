#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct Gas {
  /// The chance of a Gas Giant having a belt
  #[serde(default = "Gas::default_belt")]
  pub belt: f32,

  /// The average amount of moons a Gas Gigant has.
  #[serde(default = "Gas::default_avg_moon")]
  pub avg_moon: u32,

  /// The chance for the average to be taken for moons.
  #[serde(default = "Gas::default_chance_avg")]
  pub chance_avg: f32,

  /// The minimum amount of moons a Gas Gigant has. min < avg
  #[serde(default = "Gas::default_min_moon")]
  pub min_moon: u32,

  /// The maximum amount of moons a Gas Gigant has. avg < max
  #[serde(default = "Gas::default_max_moon")]
  pub max_moon: u32,
}

impl Gas {
  fn default_belt() -> f32 {
    0.7
  }

  fn default_avg_moon() -> u32 {
    4
  }

  fn default_chance_avg() -> f32 {
    0.32
  }

  fn default_min_moon() -> u32 {
    1
  }

  fn default_max_moon() -> u32 {
    12
  }
}

impl Default for Gas {
  fn default() -> Self {
    Gas {
      belt: Self::default_belt(),
      avg_moon: Self::default_avg_moon(),
      chance_avg: Self::default_chance_avg(),
      min_moon: Self::default_min_moon(),
      max_moon: Self::default_max_moon(),
    }
  }
}
