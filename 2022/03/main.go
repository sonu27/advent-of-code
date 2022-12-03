package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	part1()
	part2()
}

func part1() {
	file, err := os.Open("./2022/03/input.txt")
	if err != nil {
		panic(err)
	}

	var ints []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		ints = append(ints, getPriority(part1Find(line)))
	}

	sum := 0
	for _, v := range ints {
		sum += v
	}
	fmt.Println("Part 1:", sum)
}

func part1Find(line string) rune {
	first := map[rune]bool{}
	for _, r := range line[:len(line)/2] {
		first[r] = true
	}
	for _, r := range line[len(line)/2:] {
		if first[r] {
			return r
		}
	}
	return ' '
}

func part2() {
	file, err := os.Open("./2022/03/input.txt")
	if err != nil {
		panic(err)
	}

	var ints []int

	scanner := bufio.NewScanner(file)
	i := 0
	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())

		i++
		if i%3 == 0 {
			ints = append(ints, getPriority(part2Find(lines)))
			lines = []string{}
		}
	}

	sum := 0
	for _, v := range ints {
		sum += v
	}
	fmt.Println("Part 2:", sum)
}

func part2Find(lines []string) rune {
	first := map[rune]int{}
	for _, r := range lines[0] {
		first[r] = 1
	}
	for _, r := range lines[1] {
		if first[r] == 1 {
			first[r] = 2
		}
	}
	for _, r := range lines[2] {
		if first[r] == 2 {
			return r
		}
	}
	return ' '
}

func getPriority(r rune) int {
	a := int(r)
	if a > 96 {
		a -= 96 // a - 96
	} else {
		a -= 38 // A - 38
	}
	return a
}
