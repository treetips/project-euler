use crate::util::is_prime_number;

/// # [Problem 7 「10001番目の素数」](http://odz.sakura.ne.jp/projecteuler/?Problem+7)
/// ## Description
/// ```txt
/// 素数を小さい方から6つ並べると 2, 3, 5, 7, 11, 13 であり, 6番目の素数は 13 である。
/// 10 001 番目の素数を求めよ。
/// ```
fn problem_007(number: u64) -> u64 {
  let mut result = 0;
  let mut prime_number_count = 0;
  for num in 2..u64::MAX {
    if is_prime_number(num) {
      prime_number_count += 1;
    }
    if prime_number_count == number {
      result = num;
      break;
    }
  }
  println!("result=[{}]", result);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_007_1_test() {
    assert_eq!(problem_007(6), 13);
  }

  #[test]
  fn problem_007_2_test() {
    assert_eq!(problem_007(10001), 104743);
  }
}
