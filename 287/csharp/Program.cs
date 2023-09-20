

public class Solution
{
    public int FindDuplicate(int[] nums)
    {
        bool[] freq = new bool[nums.Length];
        foreach (int num in nums)
        {
            if (freq[num]) return num;
            freq[num] = true;
        }
        return -1;
    }
}