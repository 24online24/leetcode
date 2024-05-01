package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func addOneRow(root *TreeNode, val int, depth int) *TreeNode {
	if depth == 1 {
		newRoot := TreeNode{
			Val:  val,
			Left: root,
		}
		return &newRoot
	}
	queue := make([]struct {
		node  *TreeNode
		depth int
	}, 0)
	queue = append(queue, struct {
		node  *TreeNode
		depth int
	}{root, 1})
	for i := 0; i < len(queue); i++ {
		currentNode := queue[i].node
		currentDepth := queue[i].depth
		if currentDepth+1 == depth {
			nodeLeft := TreeNode{
				Val:  val,
				Left: currentNode.Left,
			}
			nodeRight := TreeNode{
				Val:   val,
				Right: currentNode.Right,
			}
			currentNode.Left = &nodeLeft
			currentNode.Right = &nodeRight
		} else {
			if currentNode.Left != nil {
				queue = append(queue, struct {
					node  *TreeNode
					depth int
				}{currentNode.Left, currentDepth + 1})
			}
			if currentNode.Right != nil {
				queue = append(queue, struct {
					node  *TreeNode
					depth int
				}{currentNode.Right, currentDepth + 1})
			}
		}
	}
	return root
}

func main() {

}
