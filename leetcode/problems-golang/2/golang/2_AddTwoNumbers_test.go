package main

import "testing"

func createlist(vals []int) *ListNode {
	if len(vals) == 0 {
		return nil
	}

	var head *ListNode

	for _, v := range vals {
		curNode := &ListNode{}
		curNode.Val = v
		curNode.Next = nil

		if head == nil {
			head = curNode
		} else {
			curNode.Next = head
			head = curNode
		}
	}
	return head
}

func showlist(t *testing.T, l *ListNode) {
	if l == nil {
		t.Log("nil list")
		return
	}

	result := make([]int, 0)
	for l != nil {
		result = append(result, l.Val)
		l = l.Next
	}
	t.Logf("list = %v\n", result)
}

func checklist(t *testing.T, l *ListNode, vals []int) {
	if l == nil {
		t.Fatal("nil list")
	}

	l = reverselist(l)

	i := 0
	valLength := len(vals)
	cur := l
	for cur != nil && i < valLength {
		if cur.Val != vals[i] {
			showlist(t, l)
			t.Fatalf("i = %v, cur.Val = %v, vals[i] = %v, vals = %v", i, cur.Val, vals[i], vals)
		}
		cur = cur.Next
		i++
	}
}

func testAddTwoNumbers(t *testing.T,
	vl1, vl2, vl3 []int) {
	l1 := createlist(vl1)
	l2 := createlist(vl2)
	l3 := addTwoNumbers(l1, l2)
	checklist(t, l3, vl3)
}

func TestAddTwoNumbers1(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{3, 4, 2},
		[]int{4, 6, 5},
		[]int{8, 0, 7},
	)
}

func TestAddTwoNumbers2(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{3, 4, 2},
		[]int{5},
		[]int{3, 4, 7},
	)
}

func TestAddTwoNumbers3(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{3, 4, 2},
		[]int{0},
		[]int{3, 4, 2},
	)
}

func TestAddTwoNumbers4(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{1, 2, 3, 4, 5, 6, 7},
		[]int{9, 8, 7, 6, 5, 4, 3, 2, 1},
		[]int{9, 8, 8, 8, 8, 8, 8, 8, 8},
	)
}

func TestAddTwoNumbers5(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{3, 4, 2},
		[]int{6, 5},
		[]int{4, 0, 7},
	)
}

func TestAddTwoNumbers6(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{3, 9, 9, 9, 1},
		[]int{9},
		[]int{4, 0, 0, 0, 0},
	)
}

func TestAddTwoNumbers7(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{9, 9, 9, 1},
		[]int{9},
		[]int{1, 0, 0, 0, 0},
	)
}

func TestAddTwoNumbers8(t *testing.T) {
	testAddTwoNumbers(t,
		[]int{1},
		[]int{9},
		[]int{1, 0},
	)
}

func TestInvalidAddTwoNumbers(t *testing.T) {
	l3 := addTwoNumbers(nil, nil)
	if l3 != nil {
		t.Fatalf("l3 = %v", l3)
	}
}
