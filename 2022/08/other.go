package main

import (
	"fmt"
	"image"
	"os"
	"strings"
)

func main() {
	input, _ := os.ReadFile("./2022/08/input.txt")

	trees := map[image.Point]int{}
	for y, s := range strings.Fields(strings.TrimSpace(string(input))) {
		for x, r := range s {
			trees[image.Point{X: x, Y: y}] = int(r - '0')
		}
	}

	part1, part2 := 0, 0
	for p, v := range trees {
		vis := 0
		score := 1

		for _, d := range []image.Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}} {
			for i := 1; ; i++ {
				if nt, ok := trees[p.Add(d.Mul(i))]; !ok {
					vis = 1
					score = score * (i - 1)
					break
				} else if nt >= v {
					score *= i
					break
				}
			}
		}

		part1 += vis
		if score > part2 {
			part2 = score
		}
	}
	fmt.Println(part1)
	fmt.Println(part2)
}
