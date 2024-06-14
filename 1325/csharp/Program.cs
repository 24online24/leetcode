var sol = new Solution();

Console.WriteLine(sol.RemoveLeafNodes(
  new TreeNode(2,
    null,
    new TreeNode(2,
      new TreeNode(2,
        new TreeNode(8, null, new TreeNode(2)),
        new TreeNode(10)), new TreeNode(2, null, new TreeNode(2)))),
  2));