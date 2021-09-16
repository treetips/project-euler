using System;
using System.Linq;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+4">Problem 4 「最大の回文積」 </a>
  /// <remarks>
  /// 左右どちらから読んでも同じ値になる数を回文数という. 2桁の数の積で表される回文数のうち, 最大のものは 9009 = 91 × 99 である.
  ///
  /// では, 3桁の数の積で表される回文数の最大値を求めよ.
  /// </remarks>
  /// </summary>
  public class P004
  {
    private static ulong Run(ulong min, ulong max)
    {
      var result = 0ul;
      for (var i = max; min <= i; i--)
      {
        for (var j = max; min <= j; j--)
        {
          var calc = i * j;
          var left = calc.ToString();
          var right = string.Concat(left.Reverse());
          if (left == right && result < calc) {
            result = calc;
          }
        }
      }
      return result;
    }

    [Test]
    public void テスト1()
    {
      Assert.AreEqual(9009, Run(10, 99));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(906609, Run(100, 999));
    }
  }
}
