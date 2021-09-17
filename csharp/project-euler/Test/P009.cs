using System;
using System.Linq;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+9">Problem 9 「特別なピタゴラス数」</a>
  /// <remarks>
  /// ピタゴラス数(ピタゴラスの定理を満たす自然数)とは a &lt; b &lt; c で以下の式を満たす数の組である.
  ///
  /// a^2 + b^2 = c^2
  /// 例えば, 3^2 + 4^2 = 9 + 16 = 25 = 5^2 である.
  ///
  /// a + b + c = 1000 となるピタゴラスの三つ組が一つだけ存在する.
  /// これらの積 abc を計算しなさい.
  /// </remarks>
  /// </summary>
  public class P009
  {
    private static long Run(long max)
    {
      var result = 0L;
      for (var a = 1; a < max; a++)
      {
        for (var b = 2; b < max; b++)
        {
          var c = max - a - b;
          if (!(Math.Abs(Math.Pow(a, 2) + Math.Pow(b, 2) - Math.Pow(c, 2)) <= 0)) continue;
          result = a * b * c;
          goto LOOP_END;
        }
      }

      LOOP_END:
      return result;
    }

    [Test]
    public void テスト1()
    {
      Assert.AreEqual(60, Run(12));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(31875000, Run(1000));
    }
  }
}
