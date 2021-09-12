package project_euler.problem;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 7 「10001番目の素数」
 *
 * <pre>
 * 素数を小さい方から6つ並べると 2, 3, 5, 7, 11, 13 であり, 6番目の素数は 13 である.
 *
 * 10 001 番目の素数を求めよ.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+7" target="_blank">問題リンク</a>
 */
public class P007 {

  private static long answer(long min, long max) {
    var result = 0L;
    var count = 0L;
    for (long i = min; i < Long.MAX_VALUE; i++) {
      if (isPrimeNumber(min, i)) {
        result = i;
        count++;
        if (count == max) {
          break;
        }
      }
    }
    return result;
  }

  private static boolean isPrimeNumber(long min, long value) {
    for (long i = min; i <= (long) Math.sqrt(value); i++) {
      if (value % i == 0) {
        return false;
      }
    }
    return true;
  }

  @Test
  void テスト1() {
    assertEquals(13, answer(2, 6));
  }

  @Test
  void テスト2() {
    assertEquals(104743, answer(2, 10_001));
  }
}
