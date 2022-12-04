package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	both()
}

func both() {
	file, err := os.Open("./2022/04/input.txt")
	if err != nil {
		panic(err)
	}

	p1, p2 := 0, 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		p1a, p1b, p2a, p2b := 0, 0, 0, 0
		_, err := fmt.Sscanf(scanner.Text(), "%d-%d,%d-%d", &p1a, &p1b, &p2a, &p2b)
		if err != nil {
			panic(err)
		}

		if p2a >= p1a && p2a <= p1b && p2b >= p1a && p2b <= p1b {
			p1++
		} else if p1a >= p2a && p1a <= p2b && p1b >= p2a && p1b <= p2b {
			p1++
		}

		if p2a >= p1a && p2a <= p1b {
			p2++
		} else if p1a >= p2a && p1a <= p2b {
			p2++
		}
	}
	fmt.Println("Part 1:", p1)
	fmt.Println("Part 2:", p2)
}
