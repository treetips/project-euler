/// # [Problem 1 「3と5の倍数」](http://odz.sakura.ne.jp/projecteuler/?Problem+1)
/// ## Description
/// ```txt
/// 10未満の自然数のうち, 3 もしくは 5 の倍数になっているものは 3, 5, 6, 9 の4つがあり, これらの合計は 23 になる。
/// 同じようにして, 1000 未満の 3 か 5 の倍数になっている数字の合計を求めよ。
/// ```
fn problem_001(from: u64, to: u64) -> u64 {
  let ret = (from..to).filter(|num| num % 3 == 0 || num % 5 == 0).sum();
  println!("result=[{}]", ret);
  ret
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_001_1_test() {
    assert_eq!(problem_001(1, 10), 23);
  }

  #[test]
  fn problem_001_2_test() {
    assert_eq!(problem_001(1, 1000), 233168);
  }
}
