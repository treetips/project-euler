package project_euler.example.util;

/**
 * 計算ユーティリティー
 *
 * @author tree
 */
public class MathUtil {

  /**
   * 素数かどうか
   *
   * @param value 判定する値
   * @return true=素数である, false=素数でない
   */
  public static boolean isPrimeNumber(long value) {
    for (long i = 2; i <= (long) Math.sqrt((double) value); i++) {
      if (value % i == 0) {
        return false;
      }
    }
    return true;
  }
}
