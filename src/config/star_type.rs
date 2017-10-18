
#[derive(Serialize, Deserialize, Debug)]
pub struct StarType {
  /// Chance for the star to spawn
  pub chance: f32,
  
  /// Chance for the star to be part of a multy star system
  pub part_multy: f32,

  /// Chance for the star to be the cause of a multy star system
  pub make_multy: f32,

  /// Chance for the star to have rocky planets
  pub rocky: f32,

  /// Chance for the star to have gas gigants
  pub gas: f32,

  /// Chance for the star to have asteroid belts
  pub belts: f32,

  pub description: String,
}
