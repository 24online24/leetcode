public class Solution
{
  public int[] KthSmallestPrimeFraction(int[] arr, int k)
  {
    var priorityQueue = new PriorityQueue<(int left, int right), decimal>();

    for (var i = 0; i < arr.Length; i++)
    {
      priorityQueue.Enqueue((i, arr.Length - 1), 1.0m * arr[i] / arr[arr.Length - 1]);
    }

    while (--k > 0)
    {
      var current = priorityQueue.Dequeue();
      current.right--;
      priorityQueue.Enqueue((current.left, current.right), 1.0m * arr[current.left] / arr[current.right]);
    }

    var result = priorityQueue.Peek();
    return [arr[result.left], arr[result.right]];
  }
}