package project_euler.example.problem;

import org.junit.jupiter.api.Test;
import project_euler.example.util.MathUtil;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 3 「最大の素因数」
 *
 * <pre>
 * 13195 の素因数は 5, 7, 13, 29 である.
 * 600851475143 の素因数のうち最大のものを求めよ.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+3" target="_blank">問題リンク</a>
 */
public class P003 {

  private static long p003(long max) {
    // 逆順ループして最初にヒットしたものが最大値
    for (long i = (long) Math.sqrt((double) max); i >= 0; i--) {
      // 約数かどうか
      if (max % i != 0) {
        continue;
      }
      // 素数かどうか
      if (MathUtil.isPrimeNumber(i)) {
        return i;
      }
    }
    return 0;
  }

  @Test
  void テスト1() {
    assertEquals(29, p003(13195L));
  }

  @Test
  void テスト2() {
    assertEquals(6857, p003(600851475143L));
  }
}