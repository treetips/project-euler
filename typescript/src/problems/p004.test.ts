/**
 * # [Problem 4 「最大の回文積」](http://odz.sakura.ne.jp/projecteuler/?Problem+4)
 * ```
 * 左右どちらから読んでも同じ値になる数を回文数という. 2桁の数の積で表される回文数のうち, 最大のものは 9009 = 91 × 99 である.
 *
 * では, 3桁の数の積で表される回文数の最大値を求めよ.
 * ```
 */
export const answer = (min: number, max: number): number => {
  let result = 0;
  for (let i = max; min < i; i--) {
    for (let j = max; min < j; j--) {
      const multiplied = i * j;
      const left = String(multiplied);
      const right = left.split("").reverse().join("");
      if (left === right && result < multiplied) {
        result = multiplied;
      }
    }
  }
  console.log(`result[${result}]`);
  return result;
};

test(`テスト1`, () => expect(answer(10, 99)).toBe(9009));
test(`テスト2`, () => expect(answer(100, 999)).toBe(906609));
