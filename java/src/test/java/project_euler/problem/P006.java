package project_euler.problem;

import org.junit.jupiter.api.Test;

import java.util.stream.LongStream;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 6 「二乗和の差」
 *
 * <pre>
 * 最初の10個の自然数について, その二乗の和は,
 *
 * 1^2 + 2^2 + ... + 10^2 = 385
 * 最初の10個の自然数について, その和の二乗は,
 *
 * (1 + 2 + ... + 10)^2 = 3025
 * これらの数の差は 3025 - 385 = 2640 となる.
 *
 * 同様にして, 最初の100個の自然数について二乗の和と和の二乗の差を求めよ.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+6" target="_blank">問題リンク</a>
 */
public class P006 {

  private static long answer(int min, int max) {
    var left = LongStream.rangeClosed(min, max).map(n -> (long) Math.pow(n, 2)).sum();
    var right = (long) Math.pow(LongStream.rangeClosed(min, max).sum(), 2);
    return right - left;
  }

  @Test
  void テスト1() {
    assertEquals(2640, answer(1, 10));
  }

  @Test
  void テスト2() {
    assertEquals(25164150, answer(1, 100));
  }
}
