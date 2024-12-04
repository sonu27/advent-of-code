package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("./2024/04/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	sum := 0
	m := [][]string{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		m = append(m, strings.Split(scanner.Text(), ""))
	}

	w := len(m[0])
	h := len(m)

	for i := 0; i < h; i++ {
		for j := 0; j < w; j++ {
			if m[i][j] != "X" {
				continue
			}

			if j+3 < w &&
				m[i][j+1] == "M" &&
				m[i][j+2] == "A" &&
				m[i][j+3] == "S" {
				sum++
			}

			if j-3 >= 0 &&
				m[i][j-1] == "M" &&
				m[i][j-2] == "A" &&
				m[i][j-3] == "S" {
				sum++
			}

			if i+3 < h &&
				m[i+1][j] == "M" &&
				m[i+2][j] == "A" &&
				m[i+3][j] == "S" {
				sum++
			}

			if i-3 >= 0 &&
				m[i-1][j] == "M" &&
				m[i-2][j] == "A" &&
				m[i-3][j] == "S" {
				sum++
			}

			if i+3 < h && j+3 < w &&
				m[i+1][j+1] == "M" &&
				m[i+2][j+2] == "A" &&
				m[i+3][j+3] == "S" {
				sum++
			}

			if i+3 < h && j-3 >= 0 &&
				m[i+1][j-1] == "M" &&
				m[i+2][j-2] == "A" &&
				m[i+3][j-3] == "S" {
				sum++
			}

			if i-3 >= 0 && j-3 >= 0 &&
				m[i-1][j-1] == "M" &&
				m[i-2][j-2] == "A" &&
				m[i-3][j-3] == "S" {
				sum++
			}

			if i-3 >= 0 && j+3 < w &&
				m[i-1][j+1] == "M" &&
				m[i-2][j+2] == "A" &&
				m[i-3][j+3] == "S" {
				sum++
			}
		}
	}

	fmt.Println("Part 1: ", sum)
	sum = 0

	for i := 1; i < h-1; i++ {
		for j := 1; j < w-1; j++ {
			if m[i][j] != "A" {
				continue
			}

			if m[i-1][j-1] == "M" &&
				m[i+1][j+1] == "S" &&
				m[i-1][j+1] == "M" &&
				m[i+1][j-1] == "S" {
				sum++
			}

			if m[i-1][j-1] == "S" &&
				m[i+1][j+1] == "M" &&
				m[i-1][j+1] == "S" &&
				m[i+1][j-1] == "M" {
				sum++
			}

			if m[i-1][j-1] == "M" &&
				m[i+1][j+1] == "S" &&
				m[i-1][j+1] == "S" &&
				m[i+1][j-1] == "M" {
				sum++
			}

			if m[i-1][j-1] == "S" &&
				m[i+1][j+1] == "M" &&
				m[i-1][j+1] == "M" &&
				m[i+1][j-1] == "S" {
				sum++
			}
		}
	}

	fmt.Println("Part 2: ", sum)
}
