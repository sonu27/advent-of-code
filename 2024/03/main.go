package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var re *regexp.Regexp

func main() {
	file, err := os.Open("./2024/03/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	re = regexp.MustCompile(`mul\((\d+),(\d+)\)`)

	text := ""
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text += scanner.Text()
	}

	fmt.Println("Part 1: ", getSum(text))

	ss := strings.Split(text, `don't()`)
	text = ss[0]

	for i := 1; i < len(ss); i++ {
		s2 := strings.Split(ss[i], `do()`)
		for j := 1; j < len(s2); j++ {
			text += s2[j]
		}
	}

	fmt.Println("Part 2: ", getSum(text))
}

func getSum(text string) int {
	sum := 0
	matches := re.FindAllStringSubmatch(text, -1)
	for _, v := range matches {
		a, _ := strconv.Atoi(v[1])
		b, _ := strconv.Atoi(v[2])

		sum += a * b
	}

	return sum
}
