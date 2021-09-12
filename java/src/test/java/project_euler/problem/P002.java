package project_euler.problem;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Problem 2 「偶数のフィボナッチ数」
 *
 * <pre>
 * フィボナッチ数列の項は前の2つの項の和である. 最初の2項を 1, 2 とすれば, 最初の10項は以下の通りである.
 *
 * 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 * 数列の項の値が400万以下のとき, 値が偶数の項の総和を求めよ.
 *
 * Note:この問題は最近更新されました. お使いのパラメータが正しいかどうか確認してください.
 * </pre>
 *
 * @see <a href="https://odz.sakura.ne.jp/projecteuler/?Problem+2" target="_blank">問題リンク</a>
 */
class P002 {

  private static int answer(int max) {
    var result = 2;
    var prevprev = 1;
    var prev = 2;
    while (true) {
      var next = prevprev + prev;
      if (next % 2 == 0) {
        result += next;
      }
      if (max <= next) {
        break;
      }
      prevprev = prev;
      prev = next;
    }
    return result;
  }

  @Test
  void テスト1() {
    assertEquals(44, answer(89));
  }

  @Test
  void テスト2() {
    assertEquals(4613732, answer(4000000));
  }
}
