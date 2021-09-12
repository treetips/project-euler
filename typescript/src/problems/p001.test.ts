import { range } from "../libs";

/**
 * # [Problem 1 「3と5の倍数」](http://odz.sakura.ne.jp/projecteuler/?Problem+1)
 * ## Description
 * ```
 * 10未満の自然数のうち, 3 もしくは 5 の倍数になっているものは 3, 5, 6, 9 の4つがあり, これらの合計は 23 になる.
 *
 * 同じようにして, 1000 未満の 3 か 5 の倍数になっている数字の合計を求めよ.
 * ```
 */
export const answer = (min: number, max: number): number => {
  return [...range(min, max)]
    .filter((num) => num % 3 === 0 || num % 5 === 0)
    .reduce((sum: number, current: number) => sum + current);
};

test(`テスト1`, () => expect(answer(0, 10)).toBe(23));
test(`テスト2`, () => expect(answer(0, 1000)).toBe(233168));
