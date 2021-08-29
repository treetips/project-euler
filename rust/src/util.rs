use bigdecimal::{BigDecimal, FromPrimitive};
use std::convert::{TryFrom, TryInto};
use std::ops::Mul;

/// # 約数を取得する
/// - @params `value` 値
/// - @return 約数
pub fn get_factors(value: u64) -> Vec<u64> {
  (1..=value).filter(|num| value % num == 0).collect()
}

/// # 素数かどうか
/// - @params `value` 値
/// - @return true=素数である, false=素数でない
pub fn is_prime_number(value: u64) -> bool {
  // 1は素数ではない
  if value == 1 {
    return false;
  }
  for num in 2..=(value as f64).sqrt() as u64 {
    // 自身と同値は素数（ループの最後＝自身）
    if value == num {
      return true;
    }
    // 割り切れる値は素数ではない
    if value % num == 0 {
      return false;
    }
  }
  true
}

/// # 回分積（パリンドローム）を取得する
/// - @params `digits` 桁数
/// - @return 回分積
pub fn get_palindrome(digits: u64) -> u64 {
  // 9でfill
  let max = "9"
    .repeat(digits.try_into().unwrap())
    .parse::<u64>()
    .unwrap();

  // 9のrpad
  let min_str = format!("{:0<width$}", "9", width = usize::try_from(digits).unwrap());
  let min = min_str.parse::<u64>().unwrap();

  for i in (min..=max).rev() {
    for j in (min..=max).rev() {
      // 91 × 99 = 9009
      let result = i * j;
      let left = result.to_string();
      let right = left.chars().rev().collect::<String>();
      if left == right {
        return result;
      }
    }
  }
  0
}

/// # 最大公約数を取得する
/// gcd = greatest common divisor
///
/// ユークリッドの互除法で求めます
pub fn gcd(min: u64, max: u64) -> u64 {
  if min % max == 0 {
    max
  } else {
    gcd(max, min % max)
  }
}

/// # 最小公倍数を取得する
/// lcm = least common multiple
pub fn lcm(min: u64, max: u64) -> u64 {
  min * max / gcd(min, max)
}

/// # pow（BigDecimal版）
/// - @params `base` 底となる数
/// - @params `exponent` 累乗する指数
/// - @return 回分積
pub fn pow(base: u128, exponent: u128) -> BigDecimal {
  let mut result: BigDecimal = BigDecimal::from_u8(1).unwrap();
  for _ in 1..=exponent {
    result = result.mul(BigDecimal::from_u128(base).unwrap());
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn get_factors_test() {
    assert_eq!(get_factors(8), vec!(1, 2, 4, 8));
    assert_eq!(get_factors(10), vec!(1, 2, 5, 10));
  }

  #[test]
  fn is_prime_number_test() {
    assert!(!is_prime_number(1));
    assert!(is_prime_number(2));
    assert!(is_prime_number(3));
    assert!(!is_prime_number(4));
    assert!(is_prime_number(5));
    assert!(!is_prime_number(6));
    assert!(is_prime_number(7));
    assert!(!is_prime_number(8));
    assert!(!is_prime_number(9));
    assert!(!is_prime_number(10));
    assert!(is_prime_number(11));
    assert!(!is_prime_number(12));
    assert!(is_prime_number(13));
    assert!(!is_prime_number(14));
    assert!(!is_prime_number(15));
    assert!(!is_prime_number(16));
    assert!(is_prime_number(17));
    assert!(!is_prime_number(18));
    assert!(is_prime_number(19));
  }

  #[test]
  fn gcd_test() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(27, 36), 9);
  }

  #[test]
  fn lcm_test() {
    assert_eq!(lcm(12, 18), 6);
    assert_eq!(lcm(27, 36), 9);
  }

  #[test]
  fn pow_test() {
    assert_eq!(pow(2, 2), BigDecimal::from_u128(4).unwrap());
    assert_eq!(pow(2, 1000), BigDecimal::from_str("10715086071862673209484250490600018105614048117055336074437503883703510511249361224931983788156958581275946729175531468251871452856923140435984577574698574803934567774824230985421074605062371141877954182153046474983581941267398767559165543946077062914571196477686542167660429831652624386837205668069376").unwrap());
  }
}
