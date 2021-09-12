/**
 * # [Problem 5 「最小の倍数」](http://odz.sakura.ne.jp/projecteuler/?Problem+5)
 * ```
 * 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
 *
 * では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.
 * ```
 */
export const answer = (min: number, max: number): number => {
  let result = 0;
  for (let i = min; i <= Number.MAX_VALUE; i++) {
    if (isDivideAll(i, min, max)) {
      result = i;
      break;
    }
  }
  return result;
};

const isDivideAll = (value: number, min: number, max: number): boolean => {
  for (let i = min; i <= max; i++) {
    if (value % i !== 0) {
      return false;
    }
  }
  return true;
};

test(`テスト1`, () => expect(answer(1, 10)).toBe(2520));
test(`テスト2`, () => expect(answer(1, 20)).toBe(232792560));
