use crate::util::is_prime_number;

/// # [Problem 3 「最大の素因数」](http://odz.sakura.ne.jp/projecteuler/?Problem+3)
/// ## Description
/// ```txt
/// 13195 の素因数は 5, 7, 13, 29 である.
/// 600851475143 の素因数のうち最大のものを求めよ.
/// ```
fn problem_003(value: u64) -> u64 {
  let mut result = 0;
  // 平方根の逆順の最初に見つかった値がmax（素数は2始まりなので1は不要）
  let range_to: u64 = (value as f64).sqrt() as u64;
  for num in (2..=range_to).rev() {
    // 約数かどうか 且つ 素数かどうか
    if value % num == 0 && is_prime_number(num) {
      result = num;
      break;
    }
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_003_1_test() {
    assert_eq!(problem_003(13195), 29);
  }

  #[test]
  fn problem_003_2_test() {
    assert_eq!(problem_003(600851475143), 6857);
  }
}
