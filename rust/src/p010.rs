use crate::util::is_prime_number;

/// # [Problem 10 「素数の和」](http://odz.sakura.ne.jp/projecteuler/?Problem+10)
/// ## Description
/// ```txt
/// 10以下の素数の和は 2 + 3 + 5 + 7 = 17 である。
/// 200万以下の全ての素数の和を求めよ。
/// ```
fn problem_010(max: u64) -> u64 {
  let mut result = 0;
  for prime_number in 2..=max {
    if is_prime_number(prime_number) {
      result += prime_number;
      continue;
    }
  }

  println!("result=[{}]", result);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_010_1_test() {
    assert_eq!(problem_010(10), 17);
  }

  #[test]
  fn problem_010_2_test() {
    assert_eq!(problem_010(2000000), 142913828922);
  }
}
