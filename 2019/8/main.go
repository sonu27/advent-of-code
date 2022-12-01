package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	txt := strings.TrimSpace(string(input))

	part2(txt, 25, 6)
}

func part2(str string, width int, height int) {
	l := 0
	h := 0
	w := 0

	image := make(map[int]map[int]map[int]int)

	for _, s := range str {
		a := string(s)
		v, _ := strconv.Atoi(a)
		if image[l][h] == nil {
			if image[l] == nil {
				image[l] = make(map[int]map[int]int)
			}
			image[l][h] = make(map[int]int)
		}

		image[l][h][w] = v

		w++
		if w == width {
			h++
			w = 0
		}

		if h == height {
			l++
			h = 0
		}
	}

	final := flatten(image, width, height)
	printImage(final, width, height)
}

func flatten(image map[int]map[int]map[int]int, width int, height int) map[int]map[int]int {
	final := make(map[int]map[int]int)
	for h := 0; h < height; h++ {
		for w := 0; w < width; w++ {
			if final[h] == nil {
				final[h] = make(map[int]int)
			}

			final[h][w] = 9
		}
	}

	size := len(image)
	for l := 0; l < size; l++ {
		for h := 0; h < height; h++ {
			for w := 0; w < width; w++ {
				if final[h][w] >= 2 {
					final[h][w] = image[l][h][w]
				}
			}
		}
	}

	return final
}

func printImage(image map[int]map[int]int, width int, height int) {
	for h := 0; h < height; h++ {
		for w := 0; w < width; w++ {
			if image[h][w] == 1 {
				fmt.Print("█")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Print("\n")
	}
}

func part1(str string, width int, height int) {
	l := 0
	h := 0
	w := 0

	layerZeros := map[int]int{
		0: 0,
	}

	image := make(map[int]map[int]map[int]int)

	for _, s := range str {
		a := string(s)
		v, _ := strconv.Atoi(a)
		if image[l][h] == nil {
			if image[l] == nil {
				image[l] = make(map[int]map[int]int)
			}
			image[l][h] = make(map[int]int)
		}

		image[l][h][w] = v

		if v == 0 {
			layerZeros[l]++
		}

		w++
		if w == width {
			h++
			w = 0
		}

		if h == height {
			l++
			h = 0
		}
	}

	lowestLayer := -1
	lowestNum := math.MaxInt8
	for i, v := range layerZeros {
		if v < lowestNum {
			lowestNum = v
			lowestLayer = i
		}
	}

	ones := countDigits(image[lowestLayer], 1)
	twos := countDigits(image[lowestLayer], 2)

	fmt.Println(ones * twos)
}

func countDigits(l map[int]map[int]int, digit int) int {
	sum := 0
	for _, v := range l {
		for _, k := range v {
			if k == digit {
				sum++
			}
		}
	}

	return sum
}
