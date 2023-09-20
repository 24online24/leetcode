public unsafe class Solution
{
    public int UniquePathsWithObstacles(int[][] obstacleGrid)
    {
        int m = obstacleGrid.Length;
        int n = obstacleGrid[0].Length;
        if (obstacleGrid[0][0] == 1)
            return 0;
        obstacleGrid[0][0] = 1;
        for (var j = 1; j < n; j++)
        {
            fixed (int* current = &obstacleGrid[0][j])
            {
                if (*current == 1)
                    *current = 0;
                else
                    *current = obstacleGrid[0][j - 1];
            }

        }
        for (var i = 1; i < m; i++)
        {
            if (obstacleGrid[i][0] == 1)
                obstacleGrid[i][0] = 0;
            else
                obstacleGrid[i][0] = obstacleGrid[i - 1][0];
        }
        for (var i = 1; i < m; i++)
            for (var j = 1; j < n; j++)
            {
                if (obstacleGrid[i][j] == 1)
                    obstacleGrid[i][j] = 0;
                else
                    obstacleGrid[i][j] = obstacleGrid[i - 1][j] + obstacleGrid[i][j - 1];
            }
        return obstacleGrid.Last().Last();

    }
}