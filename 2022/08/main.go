package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./2022/08/input.txt")
	if err != nil {
		panic(err)
	}

	var grid [][]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		strRow := strings.Split(scanner.Text(), "")
		var row []int
		for _, v := range strRow {
			i, _ := strconv.Atoi(v)
			row = append(row, i)
		}
		grid = append(grid, row)
	}

	p1(grid)
	p2(grid)
}

func p1(grid [][]int) {
	l := len(grid)
	w := len(grid[0])

	m := make([][]int, l)
	for i := 0; i < l; i++ {
		m[i] = make([]int, w)
	}

	count := 0
	for x := 0; x < w; x++ {
		h := -1
		for y := 0; y < l; y++ {
			if grid[y][x] > h {
				h = grid[y][x]
				if m[y][x] == 0 {
					count++
					m[y][x] = 1
				}
			}
		}

		h = -1
		for y := l - 1; y >= 0; y-- {
			if grid[y][x] > h {
				h = grid[y][x]
				if m[y][x] == 0 {
					count++
					m[y][x] = 1
				}
			}
		}
	}

	for y := 0; y < l; y++ {
		h := -1
		for x := 0; x < w; x++ {
			if grid[y][x] > h {
				h = grid[y][x]
				if m[y][x] == 0 {
					count++
					m[y][x] = 1
				}
			}
		}

		h = -1
		for x := w - 1; x >= 0; x-- {
			if grid[y][x] > h {
				h = grid[y][x]
				if m[y][x] == 0 {
					count++
					m[y][x] = 1
				}
			}
		}
	}

	fmt.Println("Part 1:", count)
}

func p2(grid [][]int) {
	l := len(grid)
	w := len(grid[0])

	max := 0
	for y := 1; y < l-1; y++ {
		for x := 1; x < w-1; x++ {
			score := scenicScore(grid, x, y, l, w)
			if score > max {
				max = score
			}
		}
	}

	fmt.Println("Part 2:", max)
}

func scenicScore(grid [][]int, x, y, l, w int) int {
	left := 0
	for i := x - 1; i >= 0; i-- {
		left++
		if grid[y][i] >= grid[y][x] {
			break
		}
	}

	right := 0
	for i := x + 1; i < w; i++ {
		right++
		if grid[y][i] >= grid[y][x] {
			break
		}
	}

	top := 0
	for i := y - 1; i >= 0; i-- {
		top++
		if grid[i][x] >= grid[y][x] {
			break
		}
	}

	bottom := 0
	for i := y + 1; i < l; i++ {
		bottom++
		if grid[i][x] >= grid[y][x] {
			break
		}
	}

	return left * right * top * bottom
}
