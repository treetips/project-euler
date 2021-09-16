using System;
using System.Linq;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+6">Problem 6 「二乗和の差」</a>
  /// <remarks>
  /// 最初の10個の自然数について, その二乗の和は,
  ///
  /// 1^2 + 2^2 + ... + 102 = 385
  /// 最初の10個の自然数について, その和の二乗は,
  ///
  /// (1 + 2 + ... + 10)^2 = 3025
  /// これらの数の差は 3025 - 385 = 2640 となる.
  ///
  /// 同様にして, 最初の100個の自然数について二乗の和と和の二乗の差を求めよ.
  /// </remarks>
  /// </summary>
  public class P006
  {
    private static long Run(int min, int max)
    {
      var left = Enumerable.Range(min, max).ToList().Select(n => (long) Math.Pow(n, 2)).Sum();
      var right = (long) Math.Pow(Enumerable.Range(min, max).ToList().Sum(), 2);
      return right - left;
    }

    [Test]
    public void テスト1()
    {
      Assert.AreEqual(2640, Run(1, 10));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(25164150, Run(1, 100));
    }
  }
}
