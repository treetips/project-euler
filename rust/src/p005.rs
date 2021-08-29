use crate::util::lcm;

/// # [Problem 5 「最小の倍数」](http://odz.sakura.ne.jp/projecteuler/?Problem+5)
/// ## Description
/// ```txt
/// 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
/// では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.
/// ```
// fn problem_005_1(min: u64, max: u64) -> u64 {
//   let mut result = 0;
//   'outer: for i in max..u64::MAX {
//     let mut match_number = 0;
//     for num in min..=max {
//       // マッチしなかった時点で次の数値へ
//       if i % num != 0 {
//         continue 'outer;
//       }
//       match_number += 1;
//       // 全部マッチした時点で処理終了
//       if match_number == max {
//         result = i;
//         break 'outer;
//       }
//     }
//   }

//   println!("result=[{}]", result);
//   result
// }

/// # ユークリッドの互除法バージョン
fn problem_005_2(min: u64, max: u64) -> u64 {
  let mut result = 1;
  for num in min..=max {
    result = lcm(result, num);
  }

  println!("result=[{}]", result);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_005_1_test() {
    assert_eq!(problem_005_2(1, 10), 2520);
  }

  #[test]
  fn problem_005_2_test() {
    assert_eq!(problem_005_2(1, 20), 232792560);
  }
}
