class Solution:
    def equalPairs(self, grid: list[list[int]]) -> int:
        n = len(grid)
        pairs = 0
        for row in grid:
            for j in range(n):
                for i in range(n):
                    if row[i] != grid[i][j]:
                        break
                else:
                    pairs += 1
        return pairs


sol = Solution()
print(sol.equalPairs([[3, 2, 1], [1, 7, 6], [2, 7, 7]]))
print(sol.equalPairs([[3, 1, 2, 2], [1, 4, 4, 5], [2, 4, 2, 2], [2, 4, 2, 2]]))
