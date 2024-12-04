package main

import (
	"aoc/lib"
	"bufio"
	"fmt"
	"os"
	"slices"
)

func main() {
	file, err := os.Open("./2024/02/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	p1 := 0
	p2 := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		a := lib.StringToIntSlice(scanner.Text())

		if check(a) {
			p1++
			p2++
			continue
		}

		for i := 0; i < len(a); i++ {
			b := slices.Clone(a)
			if check(slices.Delete(b, i, i+1)) {
				p2++
				break
			}
		}
	}

	fmt.Println("Part 1: ", p1)
	fmt.Println("Part 2: ", p2)
}

func check(a []int) bool {
	increasing := false
	for i := 1; i < len(a); i++ {
		diff := a[i] - a[i-1]

		if diff == 0 {
			break
		}

		if i == 1 {
			if diff > 0 {
				increasing = true
			}
		}

		if increasing && diff > 3 || increasing && diff < 0 {
			break
		}

		if !increasing && diff < -3 || !increasing && diff > 0 {
			break
		}

		if i == len(a)-1 {
			return true
		}
	}

	return false
}
