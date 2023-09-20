Solution sol = new();

int[][] mat1 = {
new int[] {1, 1, 0, 0, 0},
 new int[] {1, 1, 1, 1, 0},
 new int[] {1, 0, 0, 0, 0},
 new int[] {1, 1, 0, 0, 0},
 new int[] {1, 1, 1, 1, 1}
};
int k1 = 3;

Array.ForEach(sol.KWeakestRows(mat1, k1), x => Console.Write(x + " "));

public class Solution
{
    public int[] KWeakestRows(int[][] mat, int k)
    {
        int[,] weakestRows = new int[3, 2];

        for (int i = 0; i < mat.Length; i++)
        {
            int[] row = mat[i];
            int j = 0;
            while (j < row.Length && row[j] == 1)
            {
                j++;
            }
            Console.WriteLine(j + " " + i);
        }
        weakestRows = weakestRows.Select(row => row[0]);
        return weakestRows;
    }
}