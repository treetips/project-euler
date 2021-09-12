/// # [Problem 14 「最長のコラッツ数列」](http://odz.sakura.ne.jp/projecteuler/?Problem+14)
/// ## Description
/// ```txt
/// 正の整数に以下の式で繰り返し生成する数列を定義する.
///
/// n → n/2 (n が偶数)
///
/// n → 3n + 1 (n が奇数)
///
/// 13からはじめるとこの数列は以下のようになる.
///
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
/// 13から1まで10個の項になる. この数列はどのような数字からはじめても最終的には 1 になると考えられているが, まだそのことは証明されていない(コラッツ問題)
///
/// さて, 100万未満の数字の中でどの数字からはじめれば最長の数列を生成するか.
///
/// 注意: 数列の途中で100万以上になってもよい
/// ```
fn problem_014(min: u128, max: u128) -> u128 {
  let mut result = 0;
  let mut max_count = 0;
  for i in min..=max {
    let mut item_count = 0;
    let mut calculated = i;
    'inner: loop {
      calculated = calculate(calculated);
      item_count += 1;
      if max_count < item_count {
        max_count = item_count;
        result = i;
      }
      if calculated == 1 {
        break 'inner;
      }
    }
  }
  result
}

fn calculate(val: u128) -> u128 {
  if val % 2 == 0 {
    return val / 2;
  } else {
    return val * 3 + 1;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_014_1_test() {
    assert_eq!(problem_014(13, 13), 13);
  }

  #[test]
  fn problem_014_2_test() {
    assert_eq!(problem_014(1, 1000000), 837799);
  }
}
