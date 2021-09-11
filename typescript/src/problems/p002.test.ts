/**
 * # [Problem 2 「偶数のフィボナッチ数」](http://odz.sakura.ne.jp/projecteuler/?Problem+2)
 * ```
 * フィボナッチ数列の項は前の2つの項の和である. 最初の2項を 1, 2 とすれば, 最初の10項は以下の通りである.
 *
 * 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 * 数列の項の値が400万以下のとき, 値が偶数の項の総和を求めよ.
 *
 * Note:この問題は最近更新されました. お使いのパラメータが正しいかどうか確認してください.
 * ```
 */
export const answer = (max: number): number => {
  let result = 2;
  let prevprev = 1;
  let prev = 2;
  while (true) {
    const current = prevprev + prev;
    // 前々回
    prevprev = prev;
    // 前回
    prev = current;
    // 偶数の項のみ加算
    if (current % 2 === 0) {
      result += current;
    }
    if (max <= current) {
      break;
    }
  }

  console.log(`result[${result}]`);
  return result;
};

test(`テスト1`, () => expect(answer(89)).toBe(44));
test(`テスト2`, () => expect(answer(4000000)).toBe(4613732));
