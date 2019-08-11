package main

import "testing"

func TestKth(t *testing.T) {
	obj := Constructor(3, []int{4, 5, 8, 2})

	ret := obj.Add(3)
	if ret != 4 {
		t.Fatalf("obj = %v, ret = %v", obj, ret)
	}

	ret = obj.Add(5)
	if ret != 5 {
		t.Fatalf("obj = %v, ret = %v", obj, ret)
	}

	ret = obj.Add(10)
	if ret != 5 {
		t.Fatalf("obj = %v, ret = %v", obj, ret)
	}

	ret = obj.Add(9)
	if ret != 8 {
		t.Fatalf("obj = %v, ret = %v", obj, ret)
	}

	ret = obj.Add(4)
	if ret != 8 {
		t.Fatalf("obj = %v, ret = %v", obj, ret)
	}
}
