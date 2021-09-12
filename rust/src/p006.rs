/// # [Problem 6 「二乗和の差」](http://odz.sakura.ne.jp/projecteuler/?Problem+6)
/// ## Description
/// ```txt
/// 最初の10個の自然数について, その二乗の和は、1*1 + 2*2 + ... + 10*10 = 385
/// 最初の10個の自然数について, その和の二乗は、(1 + 2 + ... + 10)の2乗 = 3025
/// これらの数の差は 3025 - 385 = 2640 となる.
/// 同様にして, 最初の100個の自然数について二乗の和と和の二乗の差を求めよ.
/// ```
/// ## Note
/// [1,2] の場合、二乗の和 = 1*1 + 2*2 = 5、和の二乗 = (1+2) * (1+2) = 9。100%和の二乗の方が大きい。
fn problem_006(max: u64) -> u64 {
  let left = (1..=max).map(|num| num.pow(2)).sum::<u64>();
  let right = (1..=max).sum::<u64>().pow(2);
  right - left
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_006_1_test() {
    assert_eq!(problem_006(10), 2640);
  }

  #[test]
  fn problem_006_2_test() {
    assert_eq!(problem_006(100), 25164150);
  }
}
