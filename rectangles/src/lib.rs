#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle { length: 8, width: 7};
    let smaller = Rectangle { length: 5, width: 1};

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle { length: 8, width: 7 };
    let smaller = Rectangle { length: 5, width: 1 };

    assert!(!smaller.can_hold(&larger));
  }
}
