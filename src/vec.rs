use std::fmt::Debug;

use colored::*;

pub fn are_eq<T: Debug + PartialEq>(left: Vec<T>, right: Vec<T>) {
  fn compare<T: PartialEq>(left: &T, right: &T) -> (bool, String) {
    if left == right {
      (true, "ok".green().to_string())
    } else {
      (false, "fail".red().to_string())
    }
  }

  if left.len() != right.len() {
    panic!("Left is {} elements, right is {}", left.len(), right.len());
  } else {
    let zipped = left.into_iter().zip(right.into_iter());
    let mut equal = true;
    let mut content = String::from("\nL:\tR:\t\n");
    for (l, r) in zipped {
      let (eq, compare_result) = compare(&l, &r);
      equal = equal && eq;
      content.push_str(&format!("{:?}\t{:?}\t{}\n", l, r, compare_result));
    }
    if !equal {
      panic!(::prettify(&content))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn vector_length_is_checked_first() {
    let left = vec!(1, 2, 3);
    let right = vec!(4);
    are_eq(left, right);
  }

  #[test]
  #[should_panic]
  fn vector_contents_are_compared_side_by_side() {
    let left = vec!(1, 2, 3);
    let right = vec!(1, 2, 4);
    are_eq(left, right);
  }

  #[test]
  fn equal_vectors_should_pass() {
    let left = vec!(1, 2, 3);
    let right = vec!(1, 2, 3);
    are_eq(left, right);
  }
}
