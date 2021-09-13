package project_euler.problem;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 10 「素数の和」
 *
 * <pre>
 * 10以下の素数の和は 2 + 3 + 5 + 7 = 17 である.
 *
 * 200万以下の全ての素数の和を求めよ.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+10" target="_blank">問題リンク</a>
 */
public class P010 {

  private static long answer(long max) {
    var result = 0L;
    for (long i = 2; i <= max; i++) {
      if (isPrimeNumber(i)) {
        result += i;
      }
    }
    return result;
  }

  private static boolean isPrimeNumber(long value) {
    for (long i = 2; i <= Math.sqrt(value); i++) {
      if (value % i == 0) {
        return false;
      }
    }
    return true;
  }

  @Test
  void テスト1() {
    assertEquals(17L, answer(10L));
  }

  @Test
  void テスト2() {
    assertEquals(142913828922L, answer(2000000L));
  }
}
