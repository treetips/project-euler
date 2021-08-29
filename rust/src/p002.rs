/// # [Problem 2 「偶数のフィボナッチ数」](http://odz.sakura.ne.jp/projecteuler/?Problem+2)
/// ## Description
/// ```txt
/// フィボナッチ数列の項は前の2つの項の和である. 最初の2項を 1, 2 とすれば, 最初の10項は以下の通りである.
/// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
/// 数列の項の値が400万以下のとき, 値が偶数の項の総和を求めよ.
/// ```
fn problem_002(to: u32) -> u32 {
  let mut result = 2;
  let mut prevprev = 1;
  let mut prev = 2;
  loop {
    let current = prevprev + prev;
    // 前々回
    prevprev = prev;
    // 前回
    prev = current;
    // 偶数の項のみ加算
    if current % 2 == 0 {
      result += current;
    }
    if to <= current {
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
  fn problem_002_1_test() {
    assert_eq!(problem_002(89), 44);
  }

  #[test]
  fn problem_002_2_test() {
    assert_eq!(problem_002(4000000), 4613732);
  }
}
