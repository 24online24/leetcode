package main

import (
	"container/heap"
	"fmt"
)

type IndexPair struct {
	left  int
	right int
}

type QueueItem struct {
	indices  IndexPair
	fraction float32
	index    int
}

type PriorityQueue []*QueueItem

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool { return pq[i].fraction < pq[j].fraction }

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	item := x.(*QueueItem)
	item.index = len(*pq)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

func kthSmallestPrimeFraction(arr []int, k int) []int {
	pq := make(PriorityQueue, len(arr))
	for i, element := range arr {
		pq[i] = &QueueItem{indices: IndexPair{left: i, right: len(arr) - 1}, fraction: float32(element) / float32(arr[len(arr)-1]), index: i}
	}
	heap.Init(&pq)

	for k-1 > 0 {
		current := heap.Pop(&pq).(*QueueItem)
		current.indices.right--
		heap.Push(&pq, &QueueItem{indices: current.indices, fraction: float32(arr[current.indices.left]) / float32(arr[current.indices.right])})
		k--
	}

	result := heap.Pop(&pq).(*QueueItem)
	return []int{arr[result.indices.left], arr[result.indices.right]}
}

func main() {
	fmt.Println(kthSmallestPrimeFraction([]int{1, 2, 3, 5}, 3))
	fmt.Println(kthSmallestPrimeFraction([]int{1, 7}, 1))
}
