using NUnit.Framework;

namespace project_euler.Test
{
  /// <summary>
  /// <a href="http://odz.sakura.ne.jp/projecteuler/?Problem+2">Problem 2 「偶数のフィボナッチ数」</a>
  /// <remarks>
  /// フィボナッチ数列の項は前の2つの項の和である. 最初の2項を 1, 2 とすれば, 最初の10項は以下の通りである.
  ///
  /// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
  /// 数列の項の値が400万以下のとき, 値が偶数の項の総和を求めよ.
  ///
  /// Note:この問題は最近更新されました. お使いのパラメータが正しいかどうか確認してください.
  /// </remarks>
  /// </summary>
  public class P002
  {
    private static ulong Run(ulong max)
    {
      var result = 2ul;
      var prevprev = 1ul;
      var prev = 2ul;
      while (true)
      {
        var next = prevprev + prev;
        if (next % 2 == 0)
        {
          result += next;
        }

        if (max <= next)
        {
          break;
        }
        prevprev = prev;
        prev = next;
      }

      return result;
    }

    [Test]
    public void テスト1()
    {
      Assert.AreEqual(44, Run(89));
    }

    [Test]
    public void テスト2()
    {
      Assert.AreEqual(4613732, Run(4000000));
    }
  }
}
