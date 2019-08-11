package main

import "fmt"

func main() {
	fmt.Println("vim-go")
}

/**
* Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func bstFromPreorder(preorder []int) *TreeNode {
	if len(preorder) == 0 {
		return nil
	}

	if len(preorder) == 1 {
		return &TreeNode{
			Val:   preorder[0],
			Left:  nil,
			Right: nil,
		}
	}

	cur := preorder[0]
	rightStartIdx := -1
	for i := 1; i < len(preorder); i++ {
		if preorder[i] > cur {
			rightStartIdx = i
			break
		}
	}

	curNode := &TreeNode{}
	curNode.Val = cur

	if rightStartIdx == -1 {
		curNode.Left = bstFromPreorder(preorder[1:])
		curNode.Right = nil
	} else if rightStartIdx == 1 {
		curNode.Left = nil
		curNode.Right = bstFromPreorder(preorder[1:])
	} else {
		curNode.Left = bstFromPreorder(preorder[1:rightStartIdx])
		curNode.Right = bstFromPreorder(preorder[rightStartIdx:])
	}

	return curNode
}
