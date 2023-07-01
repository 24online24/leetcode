# Definition for a binary tree node.
import time


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def maxLevelSum(self, root: TreeNode) -> int:
        queue = [root]
        max_level = 1
        level = 1
        max_sum = root.val
        while queue:
            next_level = []
            sum = 0
            level += 1

            next_level_exists = False
            for node in queue:
                if node.left:
                    next_level.append(node.left)
                    sum += node.left.val
                    next_level_exists = True
                if node.right:
                    next_level.append(node.right)
                    sum += node.right.val
                    next_level_exists = True

            if next_level_exists:
                if sum > max_sum:
                    max_sum = sum
                    max_level = level
            queue = next_level

        return max_level


sol = Solution()
node = TreeNode(
    val=-100,
    left=TreeNode(
        val=-200,
        left=TreeNode(
            val=-20
        ),
        right=TreeNode(
            val=-5
        )
    ),
    right=TreeNode(
        val=-300,
        left=TreeNode(
            val=-10
        )
    )
)


print(sol.maxLevelSum(root=node))
