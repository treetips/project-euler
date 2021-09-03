use bigdecimal::ToPrimitive;
use std::convert::TryInto;

/// # [Problem 30 「各桁の5乗」](https://odz.sakura.ne.jp/projecteuler/?Problem+30)
/// ## Description
/// ```txt
/// 驚くべきことに, 各桁を4乗した数の和が元の数と一致する数は3つしかない.
///
/// 1634 = 1^4 + 6^4 + 3^4 + 4^4
/// 8208 = 8^4 + 2^4 + 0^4 + 8^4
/// 9474 = 9^4 + 4^4 + 7^4 + 4^4
/// ただし, 1=1^4は含まないものとする. この数たちの和は 1634 + 8208 + 9474 = 19316 である.
///
/// 各桁を5乗した数の和が元の数と一致するような数の総和を求めよ.
///
/// ## Node
/// 9^5 が 5桁 -> 99999    = 59049(9の5乗) * 5(桁) = 295245
/// 9^5 が 6桁 -> 999999   = 59049(9の5乗) * 6(桁) = 354294
/// 9^5 が 7桁 -> 9999999  = 59049(9の5乗) * 7(桁) = 413343
/// 7桁までいくと 413343 < 9999999 となり絶対に一致しないので、計算しなくてよくなる。
/// 6桁が調べる限界値となる。
/// ```
fn problem_030(min: u32, exponent: u32) -> u32 {
  let mut result = 0;
  for i in min..=get_max(exponent) {
    let sum = pow_every_digit(i, exponent);
    if i == sum {
      result += sum;
    }
  }
  println!("result=[{}]", result);
  result
}

fn get_max(exponent: u32) -> u32 {
  let mut result = 0;
  let min = "9"
    .repeat(exponent.try_into().unwrap())
    .parse::<i32>()
    .unwrap()
    .to_string()
    .len();
  for digit in min..1000 {
    let org_digit = "9".repeat(digit).len();
    let sum_digit = (9_i32.pow(exponent) * digit.to_i32().unwrap())
      .to_string()
      .len();
    // ぺき乗の和の桁がオリジナルの桁を超えなくなったらmax桁の割り出し終了
    if sum_digit < org_digit {
      break;
    }
    result = org_digit
  }
  "9".repeat(result).parse::<u32>().unwrap()
}

fn pow_every_digit(num: u32, exponent: u32) -> u32 {
  let mut sum = 0;
  for c in num.to_string().chars() {
    let n = c.to_string().parse::<u32>().unwrap();
    sum += n.pow(exponent);
  }
  sum
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_030_1_test() {
    assert_eq!(problem_030(2, 4), 19316);
  }

  #[test]
  fn problem_030_2_test() {
    assert_eq!(problem_030(2, 5), 443839);
  }
}
