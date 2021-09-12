/// # [Problem 21 「友愛数」](https://odz.sakura.ne.jp/projecteuler/?Problem+21)
/// ## Description
/// ```txt
/// d(n) を n の真の約数の和と定義する. (真の約数とは n 以外の約数のことである. )
/// もし, d(a) = b かつ d(b) = a (a ≠ b のとき) を満たすとき, a と b は友愛数(親和数)であるという.
///
/// 例えば, 220 の約数は 1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110 なので d(220) = 284 である.
/// また, 284 の約数は 1, 2, 4, 71, 142 なので d(284) = 220 である.
///
/// それでは10000未満の友愛数の和を求めよ.
/// ```
fn problem_021(n: u128) -> u128 {
  let mut result = 0;
  // 1は真の約数ではない、2は友愛数にならない、3から開始
  for i in 3..=n {
    let a = i;
    let b = d(a);
    if d(b) == a && a != b {
      result += i;
    }
  }
  result
}

fn d(n: u128) -> u128 {
  let mut result = 0;
  // 真の約数はnを含まない
  for i in 1..n {
    if n % i == 0 {
      result += i;
    }
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_021_1_test() {
    assert_eq!(problem_021(284), 504);
  }

  #[test]
  fn problem_021_2_test() {
    assert_eq!(problem_021(10000), 31626);
  }
}
