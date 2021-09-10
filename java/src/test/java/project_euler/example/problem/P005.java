package project_euler.example.problem;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 5 「最小の倍数」
 *
 * <pre>
 * 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
 *
 * では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+5" target="_blank">問題リンク</a>
 */
public class P005 {

  private static long answer(long min, long max) {
    var result = 0L;
    for (long i = 1; i < Long.MAX_VALUE; i++) {
      if (isDivideAll(min, max, i)) {
        result = i;
        break;
      }
    }

    return result;
  }

  private static boolean isDivideAll(long min, long max, long value) {
    for (long i = min; i < max; i++) {
      if (value % i != 0) {
        return false;
      }
    }
    return true;
  }

  @Test
  void テスト1() {
    assertEquals(2520, answer(1, 10));
  }

  @Test
  void テスト2() {
    assertEquals(232792560, answer(1, 20));
  }
}
