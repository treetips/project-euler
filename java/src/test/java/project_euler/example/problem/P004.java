package project_euler.example.problem;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 4 「最大の回文積」
 *
 * <pre>
 * 左右どちらから読んでも同じ値になる数を回文数という. 2桁の数の積で表される回文数のうち, 最大のものは 9009 = 91 × 99 である.
 *
 * では, 3桁の数の積で表される回文数の最大値を求めよ.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+4" target="_blank">問題リンク</a>
 */
public class P004 {

  private static long answer(long min, long max) {
    var result = 0L;
    for (long i = max; min <= i; i--) {
      for (long j = max; min <= j; j--) {
        var calc = i * j;
        var left = String.valueOf(calc);
        var right = new StringBuilder(left).reverse().toString();
        if (left.equals(right) && result < calc) {
          result = calc;
        }
      }
    }
    return result;
  }

  @Test
  void テスト1() {
    assertEquals(9009, answer(10, 99));
  }

  @Test
  void テスト2() {
    assertEquals(906609, answer(100, 999));
  }
}
