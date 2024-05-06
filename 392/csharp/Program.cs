// See https://aka.ms/new-console-template for more information
var sol = new Solution();
Console.WriteLine(sol.IsSubsequence("", "ahbgdc"));


public class Solution
{
    public bool IsSubsequence(string s, string t)
    {
        if (s.Length == 0) return true;
        int i = 0;
        foreach (var ch in t)
        {
            if (s[i] == ch)
            {
                i++;
                if (i == s.Length)
                {
                    return true;
                }
            }
        }
        return false;
    }
}