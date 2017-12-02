#[macro_export]
macro_rules! dice {
  ($(#[$attr:meta])* $D:ident: $E:expr) => {
    $(#[$attr])*
    pub struct $D;

    implement!($D, $E, u16);
    implement!($D, $E, u32);
    implement!($D, $E, u64);
    implement!($D, $E, usize);
    implement!($D, $E, i16);
    implement!($D, $E, i32);
    implement!($D, $E, i64);
    implement!($D, $E, isize);
  };
}

macro_rules! implement {
  ($D:ident, $E:expr, $T:ty) => {
    impl_add!($D, $E, $T);
    impl_sub!($D, $E, $T);
    impl_mul!($D, $E, $T);
    impl_div!($D, $E, $T);
    impl_into!($D, $E, $T);

    impl Dice<$T> for $D {
      fn n(self, n: $T) -> $T {
        (0..n).fold(0, |acc, _| $D + acc)
      }

      fn nkl(self, n: $T, kl: $T) -> $T {
        assert!(n > kl);
        let mut v = (0 as $T..n).map(|_| $D + 0 as $T).collect::<Vec<$T>>();
        v.sort();
        let s = v.drain(..kl as usize).sum();
        s
      }

      fn nkh(self, n: $T, kh: $T) -> $T {
        self.ndl(n, n - kh)
      }

      fn ndl(self, n: $T, dl: $T) -> $T {
        assert!(n > dl);
        let mut v = (0 as $T..n).map(|_| $D + 0 as $T).collect::<Vec<$T>>();
        v.sort();
        let s = v.drain(dl as usize..).sum();
        s
      }

      fn ndh(self, n: $T, dh: $T) -> $T {
        self.nkl(n, n - dh)
      }
    }
  };
}

macro_rules! impl_add {
  ($D:ident, $E:expr, $T:ty) => {
    impl ::std::ops::Add<$T> for $D {
      type Output = $T;

      fn add(self, other: $T) -> $T {
        ::rand::thread_rng().gen_range(1, $E + 1) + other
      }
    }
  };
}

macro_rules! impl_sub {
  ($D:ident, $E:expr, $T:ty) => {
    impl ::std::ops::Sub<$T> for $D {
      type Output = $T;

      fn sub(self, other: $T) -> $T {
        ::rand::thread_rng().gen_range(1, $E + 1) - other
      }
    }
  };
}

macro_rules! impl_mul {
  ($D:ident, $E:expr, $T:ty) => {
    impl ::std::ops::Mul<$T> for $D {
      type Output = $T;

      fn mul(self, other: $T) -> $T {
        ::rand::thread_rng().gen_range(1, $E + 1) * other
      }
    }
  };
}

macro_rules! impl_div {
  ($D:ident, $E:expr, $T:ty) => {
    impl ::std::ops::Div<$T> for $D {
      type Output = $T;

      fn div(self, other: $T) -> $T {
        ::rand::thread_rng().gen_range(1, $E + 1) / other
      }
    }
  };
}

macro_rules! impl_into {
  ($D:ident, $E:expr, $T:ty) => {
    impl ::std::convert::Into<$T> for $D {
      fn into(self) -> $T {
        ::rand::thread_rng().gen_range(1, $E + 1)
      }
    }
  };
}
