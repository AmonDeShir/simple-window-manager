
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size<T> {
  pub width: T,
  pub height: T,
}

impl<T> Sizeable for Size<T> {
  type T = T;

  fn to_tuple(&self) -> (&T, &T) {
    (&self.width, &self.height)
  }

  fn from_tuple(size: (T, T)) -> Size<T> {
    Size {
      width: size.0,
      height: size.1,
    }
  }

  fn new(width: T, height: T) -> Size<T> {
    Size {
      width,
      height,
    }
  }
}

impl Size<u16> {
  pub fn to_f64(&self) -> Size<f64> {
    Size {
      width: self.width.into(),
      height: self.width.into()
    }
  }
}

pub trait Sizeable {
  type T;

  fn new(width: Self::T, height: Self::T) -> Size<Self::T>;
  fn from_tuple(size: (Self::T, Self::T)) -> Size<Self::T>;
  fn to_tuple(&self) -> (&Self::T, &Self::T);
}
