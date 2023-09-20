var sol = new Solution();

Console.WriteLine(sol.MinOperations(new int[] { 1, 1, 4, 2, 3 }, 5));
Console.WriteLine(sol.MinOperations(new int[] { 5, 6, 7, 8, 9 }, 4));
Console.WriteLine(sol.MinOperations(new int[] { 3, 2, 20, 1, 1, 3 }, 10));

public class Solution
{
    public int MinOperations(int[] nums, int x)
    {
        if (nums.First() > x && nums.Last() > x) return -1;
        int operationsCounter = 0;
        return operationsCounter;
    }
}