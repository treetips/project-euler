using System;
using System.Linq;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+10">Problem 10 「素数の和」</a>
  /// <remarks>
  /// 10以下の素数の和は 2 + 3 + 5 + 7 = 17 である.
  ///
  /// 200万以下の全ての素数の和を求めよ.
  /// </remarks>
  /// </summary>
  public class P010
  {
    private static ulong Run(ulong max)
    {
      var result = 0ul;
      for (var i = 2ul; i <= max; i++)
      {
        if (IsPrimeNumber(i))
        {
          result += i;
        }
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
      Assert.AreEqual(17, Run(10L));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(142913828922L, Run(2000000L));
    }
  }
}
