use crate::util::pow;

/// # [Problem 16 「各位の数字の和」](https://odz.sakura.ne.jp/projecteuler/?Problem+16)
/// ## Description
/// ```txt
/// 2^15 = 32768 であり, 各位の数字の和は 3 + 2 + 7 + 6 + 8 = 26 となる.
///
/// 同様にして, 2^1000 の各位の数字の和を求めよ.
///
/// 注: Problem 20 も各位の数字の和に関する問題です。解いていない方は解いてみてください。
/// ```
fn problem_016(exponent: u128) -> u64 {
  let num_str = pow(2, exponent).to_string();
  num_str
    .chars()
    .into_iter()
    .map(|s| s.to_string().parse::<u64>().unwrap())
    .sum::<u64>()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_016_1_test() {
    assert_eq!(problem_016(15), 26);
  }

  #[test]
  fn problem_016_2_test() {
    assert_eq!(problem_016(1000), 1366);
  }
}
