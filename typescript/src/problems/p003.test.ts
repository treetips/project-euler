import { isPrimeNumber } from "../libs";

/*
http://odz.sakura.ne.jp/projecteuler/?Problem+3
13195 の素因数は 5, 7, 13, 29 である.

600851475143 の素因数のうち最大のものを求めよ.
*/
export const answer = (value: number): number => {
  let result = 0;
  for (let i = Math.trunc(Math.sqrt(value)); 2 <= value; i--) {
    if (value % i == 0 && isPrimeNumber(i)) {
      result = i;
      break;
    }
  }
  console.log(`result[${result}]`);
  return result;
};

test(`テスト1`, () => expect(answer(13195)).toBe(29));
test(`テスト2`, () => expect(answer(600851475143)).toBe(6857));
