package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func recursiveDoubling(head *ListNode) bool {
	carryOver := false
	if head.Next != nil {
		carryOver = recursiveDoubling(head.Next)
	}
	head.Val = head.Val * 2
	if carryOver {
		head.Val++
	}
	if head.Val > 9 {
		head.Val -= 10
		return true
	}
	return false
}

func doubleIt(head *ListNode) *ListNode {
	carryOver := recursiveDoubling(head)
	if carryOver {
		head = &ListNode{Val: 1, Next: head}
	}
	return head
}

func main() {

}
