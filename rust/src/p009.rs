/// # [Problem 9 「特別なピタゴラス数」](http://odz.sakura.ne.jp/projecteuler/?Problem+9)
/// ## Description
/// ```txt
/// ピタゴラス数(ピタゴラスの定理を満たす自然数)とは a < b < c で以下の式を満たす数の組である。
/// a^2 + b^2 = c^2
/// 例えば, 3^2 + 4^2 = 9 + 16 = 25 = 5^2 である。
/// a + b + c = 1000 となるピタゴラスの三つ組が一つだけ存在する。
/// これらの積 abc を計算しなさい。
/// ```
fn problem_009(sum: u64) -> u64 {
  let mut result = 0;
  'outer: for a in 1..=sum {
    'inner: for b in 1..=sum {
      // unsignedなのでマイナス値を回避
      if sum < a + b {
        continue 'inner;
      }
      let c = sum - a - b;
      // 0乗算回避
      if c == 0 {
        continue 'inner;
      }
      if a.pow(2) + b.pow(2) == c.pow(2) {
        result = a * b * c;
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
  fn problem_009_1_test() {
    assert_eq!(problem_009(12), 60);
  }

  #[test]
  fn problem_009_2_test() {
    assert_eq!(problem_009(1000), 31875000);
  }
}
