/**
 * # [Problem 7 「10001番目の素数」 ](http://odz.sakura.ne.jp/projecteuler/?Problem+7)
 * ## Description
 * ```
 * 素数を小さい方から6つ並べると 2, 3, 5, 7, 11, 13 であり, 6番目の素数は 13 である.
 *
 * 10 001 番目の素数を求めよ.
 * ```
 */
export const answer = (min: number, max: number): number => {
  let result = 0;
  let count = 0;
  for (let i = min; i < Number.MAX_VALUE; i++) {
    if (isPrimeNumber(min, i)) {
      result = i;
      count++;
      if (count === max) {
        break;
      }
    }
  }
  return result;
};

const isPrimeNumber = (min: number, value: number): boolean => {
  for (let i = min; i <= Math.sqrt(value); i++) {
    if (value % i == -0) {
      return false;
    }
  }
  return true;
};

test(`テスト1`, () => expect(answer(2, 6)).toBe(13));
test(`テスト2`, () => expect(answer(2, 10_001)).toBe(104743));
