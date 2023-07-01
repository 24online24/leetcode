from collections import defaultdict


class Solution:
    def equalPairs(self, grid: list[list[int]]) -> int:
        n = len(grid)
        pairs = 0

        columns = defaultdict(int)
        for j in range(n):
            column = []
            for i in range(n):
                column.append(grid[i][j])
            columns[str(column)] += 1
        for row in grid:
            pairs += columns[str(row)]
        return pairs



sol = Solution()
print(sol.equalPairs([[3, 2, 1], [1, 7, 6], [2, 7, 7]]))
print(sol.equalPairs([[3, 1, 2, 2], [1, 4, 4, 5], [2, 4, 2, 2], [2, 4, 2, 2]]))
