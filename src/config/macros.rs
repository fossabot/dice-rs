macro_rules! map {
  ($($id:expr => $val:expr),+) => ({
    let mut m = Map::new();
    $(
      m.insert(From::from($id), $val);
    )+
    m
  });
}
