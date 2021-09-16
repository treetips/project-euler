using System;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+3">Problem 3 「最大の素因数」 </a>
  /// <remarks>
  /// 13195 の素因数は 5, 7, 13, 29 である.
  ///
  /// 600851475143 の素因数のうち最大のものを求めよ.
  /// </remarks>
  /// </summary>
  public class P003
  {
    private static ulong Run(ulong max)
    {
      var result = 0ul;
      for (var i = (ulong) Math.Sqrt(max); 1 <= i; i--)
      {
        if (max % i != 0 || !IsPrimeNumber(i)) continue;
        result = i;
        break;
      }

      return result;
    }

    private static bool IsPrimeNumber(ulong value)
    {
      for (ulong i = 2; i <= Math.Sqrt(value); i++)
      {
        if (value % i == 0)
        {
          return false;
        }
      }

      return true;
    }

    [Test]
    public void テスト1()
    {
      Assert.AreEqual(29, Run(13195));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(6857, Run(600851475143));
    }
  }
}
