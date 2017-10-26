use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  /// The average amount of planets in a system
  #[serde(default = "Config::default_avg_planets")]
  pub avg_planets: u32,

  /// The chance for the average to be taken for planets
  #[serde(default = "Config::default_chance_avg")]
  pub chance_avg: f32,

  /// The minimum amount of planets in a system. min < avg
  #[serde(default = "Config::default_min_planets")]
  pub min_planets: u32,

  /// The maximum amount of planets in a system. avg < max
  #[serde(default = "Config::default_max_planets")]
  pub max_planets: u32,

  ///  A list of all possible star types associated to their name
  #[serde(skip_serializing_if = "Vec::is_empty", default = "Config::default_star_types")]
  pub star_types: StarTypes,

  /// Options for generating rocky planets
  #[serde(default)]
  pub rocky: Rocky,

  /// Options for generating gas giants
  #[serde(default)]
  pub gas: Gas,
}

impl Config {
  fn default_avg_planets() -> u32 {
    8
  }

  fn default_chance_avg() -> f32 {
    0.32
  }

  fn default_min_planets() -> u32 {
    2
  }

  fn default_max_planets() -> u32 {
    12
  }

  fn default_star_types() -> StarTypes {
    vec![
      StarType {
        chance: 0.00003,
        part_multy: 0.05,
        make_multy: 0.06,
        rocky: 0.3,
        gas: 0.4,
        belts: 0.2,
        cloud: 0.9,
        class: "O Class Star".into(),
        description: "O-type stars are very hot and extremely luminous, \
                      with most of their radiated output in the ultraviolet range. \
                      These are the rarest of all main-sequence stars. \
                      About 1 in 3,000,000 (0.00003%) of the main-sequence stars in the solar neighborhood are O-type stars."
          .into(),
      },
      StarType {
        chance: 0.125,
        part_multy: 0.6,
        make_multy: 0.05,
        rocky: 0.35,
        gas: 0.4,
        belts: 0.25,
        cloud: 0.75,
        class: "B Class Star".into(),
        description: "B-type stars are very luminous and blue. \
                      About 1 in 800 (0.125%) of the main-sequence stars in the solar neighborhood are B-type main-sequence objects."
          .into(),
      },
      StarType {
        chance: 0.625,
        part_multy: 0.8,
        make_multy: 0.1,
        rocky: 0.4,
        gas: 0.5,
        belts: 0.35,
        cloud: 0.65,
        class: "A Class Star".into(),
        description: "A-type stars are among the more common naked eye stars, \
                      and are white or bluish-white. \
                      About 1 in 160 (0.625%) of the main-sequence stars in the solar neighborhood are A-type stars."
          .into(),
      },
      StarType {
        chance: 3.03,
        part_multy: 0.7,
        make_multy: 0.08,
        rocky: 0.45,
        gas: 0.55,
        belts: 0.4,
        cloud: 0.6,
        class: "F Class Star".into(),
        description: "F-type stars are white in color. \
                      About 1 in 33 (3.03%) of the main-sequence stars in the solar neighborhood are F-type stars"
          .into(),
      },
      StarType {
        chance: 7.5,
        part_multy: 0.4,
        make_multy: 0.05,
        rocky: 0.6,
        gas: 0.6,
        belts: 0.2,
        cloud: 0.55,
        class: "G Class Star".into(),
        description: "Class G main-sequence stars make up about 7.5%, \
                      nearly one in thirteen, of the main-sequence stars in the solar neighborhood."
          .into(),
      },
      StarType {
        chance: 12.0,
        part_multy: 0.5,
        make_multy: 0.02,
        rocky: 0.5,
        gas: 0.4,
        belts: 0.4,
        cloud: 0.5,
        class: "K Class Star".into(),
        description: "K-type stars are orangish stars that are slightly cooler than the Sun. \
                      They make up about 12%, nearly one in eight, of the main-sequence stars in the solar neighborhood."
          .into(),
      },
      StarType {
        chance: 76.0,
        part_multy: 0.56,
        make_multy: 0.01,
        rocky: 0.8,
        gas: 0.4,
        belts: 0.3,
        cloud: 0.8,
        class: "M Class Star".into(),
        description: "Class M stars are by far the most common. \
                      About 76% of the main-sequence stars in the solar neighborhood are class M stars. \
                      However, class M main-sequence stars (red dwarfs) have such low luminosities that none are bright enough to be seen with the unaided eye, \
                      unless under exceptional conditions."
          .into(),
      },
      StarType {
        chance: 0.75,
        part_multy: 0.3,
        make_multy: 0.6,
        rocky: 0.25,
        gas: 0.21,
        belts: 0.34,
        cloud: 0.45,
        class: "D Class Star".into(),
        description: "Class D (for Degenerate) is the modern classification used for white dwarfs \
                      – low-mass stars that are no longer undergoing nuclear fusion and have shrunk to planetary size, slowly cooling down."
          .into(),
      },
      StarType {
        chance: 0.5,
        part_multy: 0.2,
        make_multy: 0.7,
        rocky: 0.2,
        gas: 0.1,
        belts: 0.3,
        cloud: 0.4,
        class: "Neutron Star".into(),
        description: "A neutron star is the collapsed core of a large star which before collapse would have had a total of between 10 and 29 solar masses. \
                      Neutron stars are the smallest and densest stars known to exist. \
                      Though neutron stars typically have a radius on the order of 10 kilometres (6.2 mi), \
                      they can have masses of about twice that of the Sun. \
                      They result from the supernova explosion of a massive star, \
                      combined with gravitational collapse, that compresses the core past the white dwarf star density to that of atomic nuclei."
          .into(),
      },
      StarType {
        chance: 0.1,
        part_multy: 0.1,
        make_multy: 0.9,
        rocky: 0.1,
        gas: 0.09,
        belts: 0.2,
        cloud: 0.28,
        class: "Black Hole".into(),
        description: "A black hole is a region of spacetime exhibiting such strong gravitational effects \
                      that nothing—not even particles and electromagnetic radiation such as light—can escape from inside it. \
                      The theory of general relativity predicts that a sufficiently compact mass can deform spacetime to form a black hole."
          .into(),
      },
    ]
  }
}

impl Default for Config {
  fn default() -> Self {
    Config {
      avg_planets: Self::default_avg_planets(),
      chance_avg: Self::default_chance_avg(),
      min_planets: Self::default_min_planets(),
      max_planets: Self::default_max_planets(),
      rocky: Rocky::default(),
      gas: Gas::default(),
      star_types: Self::default_star_types(),
    }
  }
}
