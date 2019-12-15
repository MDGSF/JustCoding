package main

import "fmt"

func main() {
	fmt.Println("vim-go")
	n1 := &ListNode{Val: 1}
	n2 := &ListNode{Val: 0}
	n3 := &ListNode{Val: 1}
	n4 := &ListNode{Val: 0}
	n1.Next = n2
	n2.Next = n3
	n3.Next = n4
	head := n1
	result := getDecimalValue(head)
	fmt.Println(result)
}

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func getDecimalValue(head *ListNode) int {
	result := 0
	cur := head
	for cur != nil {
		result <<= 1
		result |= cur.Val
		cur = cur.Next
	}
	return result
}
