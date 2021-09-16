using System;
using System.Linq;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+5">Problem 5 「最小の倍数」 </a>
  /// <remarks>
  /// 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
  ///
  /// では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.
  /// </remarks>
  /// </summary>
  public class P005
  {
    private static ulong Run(ulong min, ulong max)
    {
      var result = 0ul;
      for (var i = min; true; i++)
      {
        if (!IsDivideAll(i, min, max)) continue;
        result = i;
        break;
      }

      return result;
    }

    private static bool IsDivideAll(ulong value, ulong min, ulong max)
    {
      for (var i = min; i <= max; i++)
      {
        if (value % i != 0)
        {
          return false;
        }
      }

      return true;
    }

    [Test]
    public void テスト1()
    {
      Assert.AreEqual(2520, Run(1, 10));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(232792560, Run(1, 20));
    }
  }
}
