var sol = Solution.GroupThePeople(new[] { 3, 3, 3, 3, 3, 1, 3 });
foreach (var group in sol)
{
    foreach (var person in group)
    {
        Console.Write(person + ", ");
    }
    Console.WriteLine();
}

public class Solution
{
    public static IList<IList<int>> GroupThePeople(int[] groupSizes)
    {
        IList<IList<int>> groups = new List<IList<int>>();
        var peopleWithGroupSizes = groupSizes
            .Select((groupSize, index) => (groupSize, index))
            .OrderBy(x => x.groupSize);

        List<int> currentGroup = new();
        foreach ((int groupSize, int index) in peopleWithGroupSizes)
        {
            currentGroup.Add(index);
            if (currentGroup.Count == groupSize)
            {
                groups.Add(currentGroup);
                currentGroup = new();
            }
        }
        return groups;
    }
}