var sol = new Solution();
Console.WriteLine(sol.LongestStrChain(new string[] { "a", "b", "ba", "bca", "bda", "bdca" }));
Console.WriteLine(sol.LongestStrChain(new string[] { "xbc", "pcxbcf", "xb", "cxbc", "pcxbc" }));
Console.WriteLine(sol.LongestStrChain(new string[] { "abcd", "dbqca" }));
Console.WriteLine(sol.LongestStrChain(new string[] { "a", "b", "ab", "bac" }));

public class Solution
{
    public int LongestStrChain(string[] words)
    {
        var maxLength = 0;
        var chainLength = 1;
        var sortedWords = words.ToList();
        sortedWords.Sort((a, b) => a.Length.CompareTo(b.Length));
        for (int i = 1; i < sortedWords.Count; i++)
        {
            if (sortedWords[i - 1].IsPredecessor(sortedWords[i]))
            {
                chainLength++;
            }
            else
            {
                if (chainLength > maxLength) maxLength = chainLength;
                chainLength = 1;
            }
        }
        if (chainLength > maxLength) maxLength = chainLength;
        return maxLength;
    }


}

public static class Extensions
{
    public static bool IsPredecessor(this string wordA, string wordB)
    {
        var i = 0;
        var j = 0;
        if (wordB.Length != wordA.Length + 1) return false;
        while (i < wordA.Length && j < wordB.Length)
        {
            if (wordA[i] == wordB[j])
            {
                i++;
                j++;
            }
            else if (i == j) j++;
            else return false;
        }
        return true;
    }
}