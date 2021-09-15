using System.Linq;
using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+1">Problem 1 「3と5の倍数」</a>
  /// <remarks>
  /// 10未満の自然数のうち, 3 もしくは 5 の倍数になっているものは 3, 5, 6, 9 の4つがあり, これらの合計は 23 になる.
  ///
  /// 同じようにして, 1000 未満の 3 か 5 の倍数になっている数字の合計を求めよ.
  /// </remarks>
  /// </summary>
  public class P001
  {
    private static int Run(int min, int max)
    {
      return Enumerable.Range(min, max).ToList().Where(n => n % 3 == 0 || n % 5 == 0).Sum();
    }

    [Test]
    public void テスト1()
    {
      Assert.AreEqual(23, Run(0, 10));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(233168, Run(0, 1000));
    }
  }
}
