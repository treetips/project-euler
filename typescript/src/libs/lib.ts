/**
 * [1..10] といったrangeを生成する
 * @param start インデックス開始値
 * @param end インデックス終了値
 * @param step ステップ値
 */
export function* range(start: number, end: number, step = 1) {
  if (end === undefined) [end, start] = [start, 0];
  for (let n = start; n < end; n += step) yield n;
}

/**
 * 素数かどうか
 * @param value 検査値
 * @returns true=素数である, false=素数でない
 */
export const isPrimeNumber = (value: number): boolean => {
  for (let i = 2; i <= Math.trunc(Math.sqrt(value)); i++) {
    if (value % i === 0) {
      return false;
    }
  }
  return true;
};
