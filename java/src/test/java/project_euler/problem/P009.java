package project_euler.problem;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 9 「特別なピタゴラス数」
 *
 * <pre>
 * ピタゴラス数(ピタゴラスの定理を満たす自然数)とは a < b < c で以下の式を満たす数の組である.
 *
 * a^2 + b^2 = c^2
 * 例えば, 3^2 + 4^2 = 9 + 16 = 25 = 52 である.
 *
 * a + b + c = 1000 となるピタゴラスの三つ組が一つだけ存在する.
 * これらの積 abc を計算しなさい.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+9" target="_blank">問題リンク</a>
 */
public class P009 {

  private static long answer(int max) {
    var result = 0;
    outer:
    for (int a = 1; a <= max; a++) {
      for (int b = 2; b <= max; b++) {
        var c = max - a - b;
        if (c <= 0) {
          continue;
        }
        if (Math.pow(a, 2) + Math.pow(b, 2) == Math.pow(c, 2)) {
          result = a * b * c;
          break outer;
        }
      }
    }
    return result;
  }

  @Test
  void テスト1() {
    assertEquals(60, answer(12));
  }

  @Test
  void テスト2() {
    assertEquals(31875000, answer(1000));
  }
}
