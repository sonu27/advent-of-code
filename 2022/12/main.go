package main

import (
	"fmt"
	"image"
	"os"
	"sort"
	"strings"
)

var m = map[image.Point]struct{}{
	{X: 1, Y: 0}:  {},
	{X: -1, Y: 0}: {},
	{X: 0, Y: 1}:  {},
	{X: 0, Y: -1}: {},
}

func main() {
	b, err := os.ReadFile("./2022/12/input.txt")
	if err != nil {
		panic(err)
	}

	var grid [][]rune

	for _, line := range strings.Split(string(b), "\n") {
		var row []rune
		for _, c := range line {
			row = append(row, c)
		}
		grid = append(grid, row)
	}

	var start, end image.Point
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == 'E' {
				start.X = j
				start.Y = i
			}
			if grid[i][j] == 'S' {
				end.X = j
				end.Y = i
			}
		}
	}

	xl := len(grid[0])
	yl := len(grid)

	grid[start.Y][start.X] = 'z' + 1

	q := []image.Point{start}
	visited := make(map[image.Point]int)

	fin := false

	for len(q) > 0 && !fin {
		p := q[0]
		q = q[1:]
		for z := range m {
			n := p.Add(z)
			if n == end {
				visited[n] = visited[p] + 1
				fin = true
				break
			}
			if _, ok := visited[n]; ok {
				continue
			}
			if n.X < 0 || n.X >= xl || n.Y < 0 || n.Y >= yl {
				continue
			}
			if grid[p.Y][p.X]-grid[n.Y][n.X] > 1 {
				continue
			}
			visited[n] = visited[p] + 1
			q = append(q, n)
		}
	}

	fmt.Println("Part 1: ", visited[end])

	// Part 2
	lowest := visited[end]
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == 'a' && visited[image.Point{Y: i, X: j}] > 0 && visited[image.Point{Y: i, X: j}] < lowest {
				lowest = visited[image.Point{Y: i, X: j}]
			}
		}
	}

	fmt.Println("Part 1: ", lowest)

	// Draw
	var all []image.Point
	for v := range visited {
		all = append(all, v)
	}

	sort.Slice(all, func(i, j int) bool {
		if all[i].Y == all[j].Y {
			return all[i].X < all[j].X
		}
		return all[i].Y < all[j].Y
	})

	for i := range grid {
		for j := range grid[i] {
			if j%xl == 0 {
				fmt.Println()
			}
			fmt.Printf("%3d", visited[image.Point{Y: i, X: j}])
		}
	}
	fmt.Println()
}
