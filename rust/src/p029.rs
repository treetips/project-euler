use bigdecimal::BigDecimal;

use crate::util::pow;
use std::collections::HashSet;

/// # [Problem 29 「個別のべき乗」](https://odz.sakura.ne.jp/projecteuler/?Problem+29)
/// ## Description
/// ```txt
/// 2 ≤ a ≤ 5 と 2 ≤ b ≤ 5について, a^b を全て考えてみよう:
///
/// 2^2=4,  2^3=8,   2^4=16,  2^5=32
/// 3^2=9,  3^3=27,  3^4=81,  3^5=243
/// 4^2=16, 4^3=64,  4^4=256, 4^5=1024
/// 5^2=25, 5^3=125, 5^4=625, 5^5=3125
/// これらを小さい順に並べ, 同じ数を除いたとすると, 15個の項を得る:
///
/// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
///
/// 2 ≤ a ≤ 100, 2 ≤ b ≤ 100 で同じことをしたときいくつの異なる項が存在するか?
/// ```
fn problem_029(min: u128, max: u128) -> usize {
  let mut values: HashSet<BigDecimal> = HashSet::new();
  for a in min..=max {
    for b in min..=max {
      values.insert(pow(a, b));
    }
  }
  let result = values.len();
  println!("result=[{}]", result);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_029_1_test() {
    assert_eq!(problem_029(2, 5), 15);
  }

  #[test]
  fn problem_029_2_test() {
    assert_eq!(problem_029(2, 100), 9183);
  }
}
