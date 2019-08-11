package main

import "fmt"

func main() {
	fmt.Println("vim-go")
}

type ListNode struct {
	Val  int
	Next *ListNode
}

// addTwoNumbers 方法
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	return addTwoNumbers2(l1, l2)
}

// addTwoNumbers2 方法二
func addTwoNumbers2(l1 *ListNode, l2 *ListNode) *ListNode {
	dummyHead := &ListNode{}
	p, q, curr := l1, l2, dummyHead
	carry := 0
	for p != nil || q != nil {
		x, y := 0, 0
		if p != nil {
			x = p.Val
		}
		if q != nil {
			y = q.Val
		}

		sum := x + y + carry
		carry = sum / 10
		curr.Next = &ListNode{Val: sum % 10}
		curr = curr.Next

		if p != nil {
			p = p.Next
		}
		if q != nil {
			q = q.Next
		}
	}
	if carry > 0 {
		curr.Next = &ListNode{Val: 1}
	}
	return dummyHead.Next
}

// addTwoNumbers1 方法一
func addTwoNumbers1(l1 *ListNode, l2 *ListNode) *ListNode {
	if l1 == nil {
		return l2
	}
	if l2 == nil {
		return l1
	}

	var l3 *ListNode

	carry := 0
	for l1 != nil && l2 != nil {
		curNode := &ListNode{Val: (l1.Val + l2.Val + carry) % 10}
		if l1.Val+l2.Val+carry >= 10 {
			carry = 1
		} else {
			carry = 0
		}

		if l3 == nil {
			l3 = curNode
		} else {
			curNode.Next = l3
			l3 = curNode
		}

		l1 = l1.Next
		l2 = l2.Next
	}

	for l1 != nil {

		curNode := &ListNode{Val: (l1.Val + carry) % 10}
		if l1.Val+carry >= 10 {
			carry = 1
		} else {
			carry = 0
		}

		curNode.Next = l3
		l3 = curNode

		l1 = l1.Next
	}

	for l2 != nil {

		curNode := &ListNode{Val: (l2.Val + carry) % 10}
		if l2.Val+carry >= 10 {
			carry = 1
		} else {
			carry = 0
		}

		curNode.Next = l3
		l3 = curNode

		l2 = l2.Next
	}

	if carry == 1 {
		curNode := &ListNode{
			Val:  1,
			Next: l3,
		}
		l3 = curNode
	}

	return reverselist(l3)
}

func reverselist(l *ListNode) *ListNode {
	if l == nil || l.Next == nil {
		return l
	}

	p1 := l
	p2 := l.Next
	p1.Next = nil

	for p2.Next != nil {
		p3 := p2.Next
		p2.Next = p1
		p1 = p2
		p2 = p3
	}

	p2.Next = p1
	return p2
}
