class Solution:
    def findDuplicate(self, nums: list[int]) -> int:
        frequency = [0] * (len(nums) + 1)
        for num in nums:
            if frequency[num]:
                return num
            frequency[num] = 1
    
solution = Solution()
print(solution.findDuplicate([1,3,4,2,2]))
print(solution.findDuplicate([3,1,3,4,2]))
