package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("./2022/10/input.txt")
	if err != nil {
		panic(err)
	}

	in := strings.Split(string(b), "\n")

	x := 1
	cycle := 1
	ops := make([]int, 0)
	i := 0

	p1 := 0

	for i < len(in) || len(ops) > 0 {
		pos := (cycle - 1) % 40

		if pos == 0 {
			fmt.Printf("\n")
		}
		if pos >= x-1 && pos <= x+1 {
			fmt.Printf("#")
		} else {
			fmt.Printf(".")
		}

		switch cycle {
		case 20, 60, 100, 140, 180, 220:
			p1 += x * cycle
		}

		if len(ops) > 0 {
			x += ops[0]
			ops = ops[1:]
		} else {
			line := strings.Split(in[i], " ")
			if line[0] == "addx" {
				num, _ := strconv.Atoi(line[1])
				ops = append(ops, num)
			}
			i++
		}
		cycle++
	}
	fmt.Println(p1)
}
