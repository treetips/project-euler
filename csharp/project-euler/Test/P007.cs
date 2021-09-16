using System;
using System.Linq;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+7">Problem 7 「10001番目の素数」</a>
  /// <remarks>
  /// 素数を小さい方から6つ並べると 2, 3, 5, 7, 11, 13 であり, 6番目の素数は 13 である.
  ///
  /// 10 001 番目の素数を求めよ.
  /// </remarks>
  /// </summary>
  public class P007
  {
    private static ulong Run(ulong position)
    {
      var result = 0ul;
      var count = 0ul;
      for (var i = 2ul; i < ulong.MaxValue; i++)
      {
        if (!IsPrimeNumber(i)) continue;
        count++;
        if (count != position) continue;
        result = i;
        break;
      }

      return result;
    }

    private static bool IsPrimeNumber(ulong value)
    {
      for (var i = 2ul; i <= Math.Sqrt(value); i++)
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
      Assert.AreEqual(13, Run(6));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(104743, Run(10_001));
    }
  }
}
