/// # [Problem 12 「高度整除三角数」](http://odz.sakura.ne.jp/projecteuler/?Problem+12)
/// ## Description
/// ```txt
/// 三角数の数列は自然数の和で表わされ, 7番目の三角数は 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28 である. 三角数の最初の10項は:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
/// となる.
///
/// 最初の7項について, その約数を列挙すると, 以下のとおり.
///
///  1: 1
///  3: 1,3
///  6: 1,2,3,6
/// 10: 1,2,5,10
/// 15: 1,3,5,15
/// 21: 1,3,7,21
/// 28: 1,2,4,7,14,28
///
/// これから, 7番目の三角数である28は, 5個より多く約数をもつ最初の三角数であることが分かる.
/// では, 500個より多く約数をもつ最初の三角数はいくつか.
/// ```
fn problem_012(max_divisor_number: u64) -> u64 {
  let mut result = 0;
  let mut triangular_number = 0;
  'outer: for i in 1..u64::MAX {
    // 三角数
    triangular_number += i;
    // 約数の数
    let mut divisor_number = 0;
    for i in 1..=(triangular_number as f64).sqrt() as u64 {
      if triangular_number % i == 0 {
        // 10 / 2 = 5 余り 0, 10 / 5 = 2 余り 0
        // 両者は同じなので、折返しの値も含んでプラス2する
        divisor_number += 2;
      }
      if max_divisor_number <= divisor_number {
        result = triangular_number;
        break 'outer;
      }
    }
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_012_1_test() {
    assert_eq!(problem_012(5), 28);
  }

  #[test]
  fn problem_012_2_test() {
    assert_eq!(problem_012(500), 76576500);
  }
}
