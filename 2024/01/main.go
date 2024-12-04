package main

import (
	"aoc/lib"
	"bufio"
	"fmt"
	"os"
	"sort"
)

func main() {
	file, err := os.Open("./2024/01/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var left []int
	var right []int
	m := map[int]int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		l, r := 0, 0
		_, err := fmt.Sscanf(scanner.Text(), "%d   %d", &l, &r)
		if err != nil {
			panic(err)
		}

		left = append(left, l)
		right = append(right, r)
		m[r]++
	}

	p2 := 0
	for i := 0; i < len(left); i++ {
		p2 += left[i] * m[left[i]]
	}

	sort.Ints(left)
	sort.Ints(right)

	p1 := 0
	for i := 0; i < len(left); i++ {
		p1 += lib.Difference(left[i], right[i])
	}

	fmt.Println("Part 1: ", p1)
	fmt.Println("Part 2: ", p2)
}
