use crate::util::get_palindrome;

/// # [Problem 4 「最大の回文積」](http://odz.sakura.ne.jp/projecteuler/?Problem+4)
/// ## Description
/// ```txt
/// 左右どちらから読んでも同じ値になる数を回文数という。
/// 2桁の数の積で表される回文数のうち, 最大のものは 9009 = 91 × 99 である。
/// では, 3桁の数の積で表される回文数の最大値を求めよ。
/// ```
fn problem_004(digits: u64) -> u64 {
  get_palindrome(digits)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_004_1_test() {
    assert_eq!(problem_004(2), 9009);
  }

  #[test]
  fn problem_004_2_test() {
    assert_eq!(problem_004(3), 906609);
  }
}
