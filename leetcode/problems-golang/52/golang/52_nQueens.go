package main

import (
	"fmt"
)

func main() {
	fmt.Println("vim-go")
}

/*
以 4 皇后举例说明问题：

-----------------
| A | B | C | D |   A:(0, 0), B:(0, 1), C:(0, 2), D:(0, 3)
-----------------
| E | F | G | H |   E:(1, 0), F:(1, 1), G:(1, 2), H:(1, 3)
-----------------
| I | J | K | L |   I:(2, 0), J:(2, 1), K:(2, 2), L:(2, 3)
-----------------
| M | N | O | P |   M:(3, 0), N:(3, 1), O:(3, 2), P:(3, 3)
-----------------

我们定义 row[i], col[j], pie[i+j], na[i-j]

row 就是已经被占领的每一行

col 就是已经被占领的每一列

pie 就是从 B 到 M 这个方向的斜线
	同一条斜线上的每个位置的 i+j 是相同的

na  就是从 A 到 P 这个方向的斜线
	同一条斜线上的每个位置的 i-j 是相同的

这 4 个数组用来保存已经被其他皇后占领的位置，也就是不能使用的位置。

伪代码：

for i = 0; i < rowNum; i++ {
	for j = 0; j < colNum; j++ {
		check position(i, j) is valid or not
	}
}

*/

func totalNQueens(n int) int {
	return totalNQueens1(n)
}

type TContext1 struct {
	n      int
	curRow int
	result int
	row    map[int]struct{}
	col    map[int]struct{}
	pie    map[int]struct{}
	na     map[int]struct{}
}

func totalNQueens1(n int) int {
	ctx := &TContext1{
		n:      n,
		curRow: 0,
		result: 0,
		row:    make(map[int]struct{}),
		col:    make(map[int]struct{}),
		pie:    make(map[int]struct{}),
		na:     make(map[int]struct{}),
	}
	solveNQueens1Inner(ctx)
	return ctx.result
}

func solveNQueens1Inner(ctx *TContext1) {
	if ctx.curRow >= ctx.n {
		ctx.result++
		return
	}

	for curCol := 0; curCol < ctx.n; curCol++ {
		if !isPositionValid(ctx, curCol) {
			continue
		}

		ctx.row[ctx.curRow] = struct{}{}
		ctx.col[curCol] = struct{}{}
		ctx.pie[ctx.curRow+curCol] = struct{}{}
		ctx.na[ctx.curRow-curCol] = struct{}{}

		ctx.curRow++
		solveNQueens1Inner(ctx)
		ctx.curRow--

		delete(ctx.row, ctx.curRow)
		delete(ctx.col, curCol)
		delete(ctx.pie, ctx.curRow+curCol)
		delete(ctx.na, ctx.curRow-curCol)
	}
}

func isPositionValid(ctx *TContext1, curCol int) bool {
	if _, ok := ctx.row[ctx.curRow]; ok {
		return false
	}
	if _, ok := ctx.col[curCol]; ok {
		return false
	}
	if _, ok := ctx.pie[ctx.curRow+curCol]; ok {
		return false
	}
	if _, ok := ctx.na[ctx.curRow-curCol]; ok {
		return false
	}
	return true
}
