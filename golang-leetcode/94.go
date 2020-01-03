package main

import "fmt"

func main() {
	fmt.Println("vim-go")
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderTraversal(root *TreeNode) []int {
	result := make([]int, 0)
	stack := make([]*TreeNode, 0)
	currNode := root
	for currNode != nil || len(stack) > 0 {
		for currNode != nil {
			stack = append(stack, currNode)
			currNode = currNode.Left
		}
		currNode = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		result = append(result, currNode.Val)
		currNode = currNode.Right
	}
	return result
}

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func inorderTraversal_01(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	leftSubTree := inorderTraversal(root.Left)
	rightSubTree := inorderTraversal(root.Right)
	return append(append(leftSubTree, root.Val), rightSubTree...)
}
