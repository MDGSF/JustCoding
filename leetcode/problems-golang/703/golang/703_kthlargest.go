package main

import (
	"container/heap"
	"fmt"
)

func main() {
	fmt.Println("vim-go")
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * obj := Constructor(k, nums);
 * param_1 := obj.Add(val);
 */

type myHeap []int

func (h *myHeap) Len() int {
	return len(*h)
}

func (h *myHeap) Less(i, j int) bool {
	return (*h)[i] < (*h)[j]
}

func (h *myHeap) Swap(i, j int) {
	(*h)[i], (*h)[j] = (*h)[j], (*h)[i]
}

// add x as element Len()
func (h *myHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}

// remove and return element Len() - 1.
func (h *myHeap) Pop() (v interface{}) {
	*h, v = (*h)[:h.Len()-1], (*h)[h.Len()-1]
	return
}

type KthLargest struct {
	K int
	H *myHeap
}

func Constructor(k int, nums []int) KthLargest {
	c := KthLargest{}
	c.K = k
	c.H = new(myHeap)
	for _, num := range nums {
		c.H.Push(num)
	}
	heap.Init(c.H)
	for c.H.Len() > k {
		heap.Pop(c.H)
	}
	return c
}

func (this *KthLargest) Add(val int) int {
	if this.H.Len() < this.K {
		heap.Push(this.H, val)
	} else if val > (*this.H)[0] {
		heap.Pop(this.H)
		heap.Push(this.H, val)
	}
	return (*this.H)[0]
}
