using System;
using System.Linq;
using System.Collections.Generic;
using System.IO;
using System.Text;

namespace day_10
{
  static class Primes {
    public static IEnumerable<int> Below(int max) {
      var notPrime = new bool[max-2];
      for (int i = 2; i < max; ++i) {
        if (notPrime[i-2]) continue;
        yield return i;
        for(var j = i*i; j < max; j += i) {
          notPrime[j-2] = true;
        }
      }
    }
  }

  class Board {
    public readonly int[,] Cells;

    public int Width { get; }
    public int Height { get; }
    
    public Board(string input) {
      var lines = input.Split("\n", StringSplitOptions.RemoveEmptyEntries);
      if (lines.Length == 0) {
        throw new ArgumentException("Board is empty");
      }
      Height = lines.Length;
      Width = lines[0].Trim().Length;
      Cells = new int[Width, Height];

      for (var y = 0; y < Height; ++y) {
        var line = lines[y].Trim();
        if (line.Length != Width) {
          throw new ArgumentException(
            $"All rows in input must have the same length ({line.Length} vs {Width})",
            nameof(input));
        }
        for (var x = 0; x < Width; ++x) {
          var value = line[x] == '#' ? 1 : 0;
          Cells[x,y] = value;
        }
      }
    }

    public Board(Board board) {
      Width = board.Width;
      Height = board.Height;
      Cells = new int[Width, Height];
      Array.Copy(board.Cells, Cells, Cells.LongLength);
    }

    public Board Clone() => new Board(this);

    public bool IsInBounds(Point p) {
      return
        p.X >= 0 && p.X < Width && 
        p.Y >= 0 && p.Y < Height;
    }

    public int Get(Point p) {
      if(!IsInBounds(p)) {
        return 0;
      }
      return Cells[p.X, p.Y];
    }

    public void Set(Point p, int value) {
      if(!IsInBounds(p)) {
        return;
      }
      Cells[p.X, p.Y] = value;
    }

    public override string ToString() {
      var builder = new StringBuilder();
      for (var y = 0; y < Height; ++y) {
        if (y > 0) {
          builder.Append("\n");
        }
        for (var x = 0; x < Width; ++x) {
          builder.Append(Cells[x,y].ToString());
        }
      }
      return builder.ToString();
    }
  }

  struct Point {
    public int X, Y;
    public Point(int x, int y) {
      X = x;
      Y = y;
    }

    public Point((int, int) p)
      : this (p.Item1, p.Item2)
      {}

    public static implicit operator Point((int, int) p) =>
      new Point(p);

    public static implicit operator ValueTuple<int, int>(Point point) =>
      (point.X, point.Y);

    public static Point operator + (Point p0, Point p1) =>
      new Point(p0.X + p1.X, p0.Y + p1.Y);

    public static Point operator - (Point p0, Point p1) =>
      new Point(p0.X - p1.X, p0.Y - p1.Y);

    public static Point operator *(Point p, int m) =>
      new Point(p.X * m, p.Y * m);

    public static Point operator *(Point p0, Point p1) =>
      new Point(p0.X * p1.X, p0.Y * p1.X);

    public override string ToString() => $"({X},{Y})";
  }

  class Program
  {
    static void Main(string[] args)
    {
      var asciiBoard = @"
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
".Trim();

    var board = new Board(asciiBoard);

      var primes = Primes.Below(Math.Max(board.Width, board.Height)+1).ToArray();
      var primeAngles =
        from p1 in primes
        from p2 in primes
        where p1 != p2
        select new Point(p1, p2);
        
      var startAngles =
        Enumerable.Concat(
          new[] { (1, 0), (0, 1), (1, 1) }.Select(x => new Point(x)),
          primeAngles
        ).ToArray();
          
      var start = new Point(3, 5);
      foreach (var delta in startAngles) {
        var offset = start + delta;
        while (board.IsInBounds(offset)) {
          if (board.Get(offset) > 0) {
            board.Set(offset, 9);
            break;
          }

          offset = offset + delta;
        }
      }

      Console.WriteLine(board);
    }
  }
}
