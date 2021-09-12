use std::ops::{Div, Mul};

use bigdecimal::{BigDecimal, FromPrimitive};

/// # [Problem 15 「格子経路」](https://odz.sakura.ne.jp/projecteuler/?Problem+15)
/// ## Description
/// ```txt
/// 2×2 のマス目の左上からスタートした場合, 引き返しなしで右下にいくルートは 6 つある.
///
/// では, 20×20 のマス目ではいくつのルートがあるか.
/// ```
///
/// ## Note
/// 組み合わせの公式を使う。 https://rikeilabo.com/formula-and-diferrence-of-permutation-combination
/// - → と ↓ の2種類しか使わない。
/// - →も↓も、それぞれ2個づつしか置けない。
/// - →を2個選ぶと、自動的に↓の2個が決定する。
/// - 4個から2個選ぶ、という組み合わせ。
/// - nCr -> 4C2
///
/// 4 x 3
/// ----- -> 12 / 2 = 6通り
/// 2 x 1
///
/// 40 x 39 x ... 21
/// ----------------
/// 20 x 19 x .... 1
fn problem_015(x: u128, y: u128) -> String {
  let n = x + y;
  let r = n / 2;

  let mut numerator: BigDecimal = BigDecimal::from_u8(1).unwrap();
  for i in r + 1..=n {
    numerator = numerator.mul(BigDecimal::from_u128(i).unwrap());
  }

  let mut denominator: BigDecimal = BigDecimal::from_u8(1).unwrap();
  for i in 1..=r {
    denominator = denominator.mul(BigDecimal::from_u128(i).unwrap());
  }

  numerator.div(denominator).to_string()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_015_1_test() {
    assert_eq!(problem_015(2, 2), "6");
  }

  #[test]
  fn problem_015_2_test() {
    assert_eq!(problem_015(20, 20), "137846528820");
  }
}
