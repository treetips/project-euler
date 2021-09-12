import { rangeClosed } from "../libs";
/**
 * # [Problem 6 「二乗和の差」](http://odz.sakura.ne.jp/projecteuler/?Problem+6)
 * ## Description
 * ```
 * 最初の10個の自然数について, その二乗の和は,
 *
 * 1^2 + 2^2 + ... + 102 = 385
 * 最初の10個の自然数について, その和の二乗は,
 *
 * (1 + 2 + ... + 10)^2 = 3025
 * これらの数の差は 3025 - 385 = 2640 となる.
 *
 * 同様にして, 最初の100個の自然数について二乗の和と和の二乗の差を求めよ.
 * ```
 */
export const answer = (min: number, max: number): number => {
  const left = [...rangeClosed(min, max)]
    .map((n) => Math.pow(n, 2))
    .reduce((prev, current) => prev + current);
  const right = Math.pow(
    [...rangeClosed(min, max)].reduce((prev, current) => prev + current),
    2
  );
  return right - left;
};

test(`テスト1`, () => expect(answer(1, 10)).toBe(2640));
test(`テスト2`, () => expect(answer(1, 100)).toBe(25164150));
