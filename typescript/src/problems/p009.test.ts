/**
 * # [Problem 9 「特別なピタゴラス数」](http://odz.sakura.ne.jp/projecteuler/?Problem+9)
 * ## Description
 * ```
 * ピタゴラス数(ピタゴラスの定理を満たす自然数)とは a < b < c で以下の式を満たす数の組である.
 *
 * a^2 + b^2 = c^2
 * 例えば, 3^2 + 4^2 = 9 + 16 = 25 = 52 である.
 *
 * a + b + c = 1000 となるピタゴラスの三つ組が一つだけ存在する.
 * これらの積 abc を計算しなさい.
 * ```
 */
export const answer = (max: number): number => {
  let result = 0;
  for (let a = 1; a <= max; a++) {
    for (let b = 2; b <= max; b++) {
      let c = max - a - b;
      if (c <= 0) {
        continue;
      }
      if (Math.pow(a, 2) + Math.pow(b, 2) == Math.pow(c, 2)) {
        result = a * b * c;
        break;
      }
    }
  }
  return result;
};

test(`テスト1`, () => expect(answer(12)).toBe(60));
test(`テスト2`, () => expect(answer(1000)).toBe(31875000));
