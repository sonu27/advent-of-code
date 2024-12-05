package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("./2024/05/input.txt")
	if err != nil {
		panic(err)
	}

	m := map[int]map[int]bool{}

	mySort := func(a, b int) int {
		mm, ok := m[a]
		if !ok {
			return 0
		}

		if mm[b] {
			return -1
		}

		return 1
	}

	p1 := 0
	p2 := 0

	ss := strings.Split(string(b), "\n")
	for _, v := range ss {
		if v == "" {
			continue
		}

		var l, r int
		_, err := fmt.Sscanf(v, "%d|%d", &l, &r)
		if err != nil {
			s := strings.Split(v, ",")
			var n []int
			for _, v := range s {
				i, _ := strconv.Atoi(v)
				n = append(n, i)

			}

			if slices.IsSortedFunc(n, mySort) {
				p1 += n[len(n)/2]
			} else {
				slices.SortStableFunc(n, mySort)
				p2 += n[len(n)/2]
			}
		}

		_, ok := m[l]
		if !ok {
			m[l] = map[int]bool{}
		}
		m[l][r] = true
	}

	fmt.Println("Part 1: ", p1)
	fmt.Println("Part 2: ", p2)
}
