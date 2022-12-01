package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

func main() {
	file, err := os.Open("./2022/01/input.txt")
	if err != nil {
		panic(err)
	}
	var counts []int

	scanner := bufio.NewScanner(file)
	count := 0
	for scanner.Scan() {
		var number int
		_, err := fmt.Sscanf(scanner.Text(), "%d", &number)
		if err != nil && err.Error() == "EOF" {
			counts = append(counts, count)
			count = 0
			continue
		}
		count += number
	}

	sort.Slice(counts, func(i, j int) bool {
		return counts[i] > counts[j]
	})

	fmt.Println("Part 1:", counts[0])
	fmt.Println("Part 1:", counts[0]+counts[1]+counts[2])
}
