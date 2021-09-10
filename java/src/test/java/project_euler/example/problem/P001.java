package project_euler.example.problem;

import org.junit.jupiter.api.Test;

import java.util.stream.IntStream;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 1 「3と5の倍数」
 *
 * <pre>
 * 10未満の自然数のうち, 3 もしくは 5 の倍数になっているものは 3, 5, 6, 9 の4つがあり, これらの合計は 23 になる.
 * 同じようにして, 1000 未満の 3 か 5 の倍数になっている数字の合計を求めよ.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+1" target="_blank">問題リンク</a>
 */
class P001 {

  private static int answer(int max) {
    return IntStream.range(1, max).filter(i -> i % 3 == 0 || i % 5 == 0).sum();
  }

  @Test
  void テスト1() {
    assertEquals(23, answer(10));
  }

  @Test
  void テスト2() {
    assertEquals(233168, answer(1000));
  }
}
