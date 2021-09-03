use bigdecimal::{BigDecimal, FromPrimitive};

/// # [Problem 25 「1000桁のフィボナッチ数」](https://odz.sakura.ne.jp/projecteuler/?Problem+25)
/// ## Description
/// ```txt
/// フィボナッチ数列は以下の漸化式で定義される:
///
/// F&sub{n}; = F&sub{n-1}; + F&sub{n-2};, ただし F&sub{1}; = 1, F&sub{2}; = 1.
/// 最初の12項は以下である.
///
/// F&sub{1}; = 1
/// F&sub{2}; = 1
/// F&sub{3}; = 2
/// F&sub{4}; = 3
/// F&sub{5}; = 5
/// F&sub{6}; = 8
/// F&sub{7}; = 13
/// F&sub{8}; = 21
/// F&sub{9}; = 34
/// F&sub{10}; = 55
/// F&sub{11}; = 89
/// F&sub{12}; = 144
/// 12番目の項, F&sub{12};が3桁になる最初の項である.
///
/// 1000桁になる最初の項の番号を答えよ.
/// ```
fn problem_025(digit: usize) -> String {
  let mut result: BigDecimal = BigDecimal::from_u8(0).unwrap();
  let mut prevprev: BigDecimal = BigDecimal::from_u8(1).unwrap();
  let mut prev: BigDecimal = BigDecimal::from_u8(1).unwrap();
  for i in 3..usize::MAX {
    let next = &prevprev + &prev;
    if digit <= next.to_string().len() {
      result = BigDecimal::from_usize(i).unwrap();
      break;
    }
    prevprev = prev;
    prev = next;
  }
  println!("result=[{}]", result);
  result.to_string()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_025_1_test() {
    assert_eq!(problem_025(3), "12");
  }

  #[test]
  fn problem_025_2_test() {
    assert_eq!(problem_025(1000), "4782");
  }
}
