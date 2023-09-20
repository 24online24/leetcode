Solution sol = new();
var r1 = sol.SummaryRanges(new[] { 0, 1, 2, 4, 5, 7 }).ToList();
r1.ForEach(r => Console.WriteLine($"{r}, "));

var r2 = sol.SummaryRanges(new[] { 0, 2, 3, 4, 6, 8, 9 }).ToList();
r2.ForEach(r => Console.WriteLine($"{r}, "));

var empty = new List<int>();
var r3 = sol.SummaryRanges(empty.ToArray()).ToList();
r3.ForEach(r => Console.WriteLine($"{r}, "));

public class Solution
{
    public IList<string> SummaryRanges(int[] nums)
    {
        IList<string> ranges = new List<string>();
        if (nums.Length != 0)
        {
            int first = nums[0];
            int previous = nums[0];
            foreach (var num in nums)
            {
                if (num - previous > 1)
                {
                    ranges.Add(previous != first ? $"{first}->{previous}" : $"{first}");
                    first = num;
                }
                previous = num;
            }
            ranges.Add(previous != first ? $"{first}->{previous}" : $"{first}");
        }
        return ranges;
    }
}