from math import factorial

MOD = 10 ** 9 + 7


class Solution:
    def numOfWays(self, nums: list[int]) -> int:

        def combination(nl, nr):
            return factorial(nl+nr) // (factorial(nl) * factorial(nr))

        def ways(arr):
            # print(arr)
            if len(arr) <= 2:
                return 1

            root = arr[0]

            left = [num for num in arr if num < root]
            right = [num for num in arr if num > root]

            ans_l = ways(left) % MOD
            ans_r = ways(right) % MOD
            print('Before combinations')
            comb = combination(len(left), len(right)) % MOD
            print('After combinations')
            print(arr)
            print(ans_l, ans_r, comb, sep=', ')

            lr_mult = (ans_l * ans_r) % MOD
            print(lr_mult)
            final_mult = (lr_mult * comb) % MOD
            print(final_mult)
            return final_mult

        return (ways(nums) - 1) % MOD


# nums3 = [1, 2, 3]

nums4 = [74, 24, 70, 11, 6, 4, 59, 9, 36, 82, 80, 30, 46, 31, 22, 34, 8, 69, 32, 57, 18, 21, 37, 83, 55, 38, 41, 72, 48, 65, 27, 60, 73, 58, 68, 50, 16, 77, 75, 20,
         81, 3, 61, 13, 10, 29, 62, 49, 12, 66, 39, 45, 28, 40, 42, 52, 78, 56, 44, 17, 14, 67, 35, 26, 19, 5, 63, 51, 43, 23, 79, 2, 54, 47, 76, 53, 7, 25, 64, 33, 1, 15, 71]
sol = Solution()
# print(sol.numOfWays(nums=nums3))
print(sol.numOfWays(nums=nums4))
