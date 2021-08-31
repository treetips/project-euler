use bigdecimal::{BigDecimal, FromPrimitive};
use std::ops::Mul;

/// # [Problem 20 「各位の数字の和 2」](https://odz.sakura.ne.jp/projecteuler/?Problem+20)
/// ## Description
/// ```txt
/// n × (n - 1) × ... × 3 × 2 × 1 を n! と表す.
///
/// 例えば, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800 となる.
/// この数の各桁の合計は 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27 である.
///
/// では, 100! の各位の数字の和を求めよ.
///
/// 注: Problem 16 も各位の数字の和に関する問題です。解いていない方は解いてみてください。
/// ```
fn problem_020(n: u128) -> u128 {
  let mut divide = BigDecimal::from_u128(n).unwrap();
  for i in 1..n {
    divide = divide.mul(BigDecimal::from_u128(n - i).unwrap());
  }
  let result = divide
    .to_string()
    .chars()
    .into_iter()
    .map(|v| v.to_string().parse::<u128>().unwrap())
    .sum();
  println!("result=[{}]", result);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_020_1_test() {
    assert_eq!(problem_020(10), 27);
  }

  #[test]
  fn problem_020_2_test() {
    assert_eq!(problem_020(100), 648);
  }
}
