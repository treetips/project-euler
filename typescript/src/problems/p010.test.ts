/**
 * # [Problem 10 「素数の和」](http://odz.sakura.ne.jp/projecteuler/?Problem+10)
 * ## Description
 * ```
 * 10以下の素数の和は 2 + 3 + 5 + 7 = 17 である.
 *
 * 200万以下の全ての素数の和を求めよ.
 * ```
 */
export const answer = (max: number): number => {
  let result = 0;
  for (let i=2; i<=max; i++) {
    if (isPrimeNumber(i)) {
      result += i;
    }
  }
  return result;
};

const isPrimeNumber = (value: number): boolean => {
  for(let i = 2; i <= Math.sqrt(value); i++) {
    if (value % i === 0) {
      return false;
    }
  }
  return true;
}

test(`テスト1`, () => expect(answer(10)).toBe(17));
test(`テスト2`, () => expect(answer(2000000)).toBe(142913828922));
