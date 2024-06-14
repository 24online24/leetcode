
public class TreeNode
{
  public int val;
  public TreeNode left;
  public TreeNode right;
  public TreeNode(int val = 0, TreeNode left = null, TreeNode right = null)
  {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

public class Solution
{
  public TreeNode RemoveLeafNodes(TreeNode root, int target)
  {
    if (root.left is not null)
    {
      root.left = RemoveLeafNodes(root.left, target);
    }
    if (root.right is not null)
    {
      root.right = RemoveLeafNodes(root.right, target);
    }
    if (root.left is null && root.right is null && root.val == target)
    {
      return null;
    }
    return root;
  }
}