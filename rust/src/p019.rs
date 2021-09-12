use chrono::{Datelike, NaiveDate, Weekday};

/// # [Problem 19 「日曜日の数え上げ」](https://odz.sakura.ne.jp/projecteuler/?Problem+19)
/// ## Description
/// ```txt
/// 次の情報が与えられている.
///
/// * 1900年1月1日は月曜日である.
/// * 9月, 4月, 6月, 11月は30日まであり, 2月を除く他の月は31日まである.
/// * 2月は28日まであるが, うるう年のときは29日である.
/// * うるう年は西暦が4で割り切れる年に起こる. しかし, 西暦が400で割り切れず100で割り切れる年はうるう年でない.
///
/// 20世紀（1901年1月1日から2000年12月31日）中に月の初めが日曜日になるのは何回あるか?
/// ```
///
/// # Note
/// 動的計画法
fn problem_019() -> u32 {
  let mut result = 0;
  let mut datetime = NaiveDate::from_ymd(1901, 1, 1);
  while datetime.year() <= 2000 {
    if datetime.weekday() == Weekday::Sun {
      result += 1;
    }
    let mut year = datetime.year();
    let mut month = datetime.month();
    if datetime.month() == 12 {
      year += 1;
      month = 1;
    } else {
      month += 1;
    }
    datetime = NaiveDate::from_ymd(year, month, datetime.day());
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_019_1_test() {
    assert_eq!(problem_019(), 171);
  }
}
