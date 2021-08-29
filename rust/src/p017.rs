#[rustfmt::skip]
const GROUP1: [&str; 20] = [
  "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
];
const GROUP2: [&str; 10] = [
  "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

/// # [Problem 17 「数字の文字数」](https://odz.sakura.ne.jp/projecteuler/?Problem+17)
/// ## Description
/// ```txt
/// 1 から 5 までの数字を英単語で書けば one, two, three, four, five であり, 全部で 3 + 3 + 5 + 4 + 4 = 19 の文字が使われている.
///
/// では 1 から 1000 (one thousand) までの数字をすべて英単語で書けば, 全部で何文字になるか.
///
/// 注: 空白文字やハイフンを数えないこと. 例えば, 342 (three hundred and forty-two) は 23 文字, 115 (one hundred and fifteen) は20文字と数える. なお, "and" を使用するのは英国の慣習.
/// ```
fn problem_017(max: u128) -> usize {
  let mut result = 0;
  for i in 1..=max as usize {
    result += to_english(i).len()
  }
  println!("result=[{}]", result);
  result
}

fn to_english(i: usize) -> String {
  if (0..=19).contains(&i) {
    GROUP1[i].to_string()
  } else if (20..=99).contains(&i) {
    let prefix = GROUP2[i / 10];
    let suffix = if i % 10 != 0 {
      GROUP1[i % 10].to_string()
    } else {
      "".to_string()
    };
    format!("{}{}", prefix, suffix)
  } else if (100..=999).contains(&i) {
    let prefix = GROUP1[i / 100];
    let suffix = if i % 100 != 0 {
      format!("and{}", to_english(i % 100))
    } else {
      "".to_string()
    };
    format!("{}hundred{}", prefix, suffix)
  } else if (1000..=999999).contains(&i) {
    let prefix = to_english(i / 1000);
    let suffix = if i % 1000 != 0 {
      format!("and{}", to_english(i % 1000))
    } else {
      "".to_string()
    };
    format!("{}thousand{}", prefix, suffix)
  } else {
    panic!("IllegalArgumentException");
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn problem_017_1_test() {
    assert_eq!(problem_017(5), 19);
  }

  #[test]
  fn problem_017_2_test() {
    assert_eq!(problem_017(1000), 21124);
  }
}
