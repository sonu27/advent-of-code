package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	txt := strings.TrimSpace(string(input))
	strArr := strings.Split(txt, ",")

	part2(strArr)
}

func part1(strArr []string) {
	m := convertToMap(strArr)
	ans := start(m, 12, 2)
	fmt.Println(ans)
}

func part2(strArr []string) {
	a := 0
	n := 0
	v := 0
	ans := 0
	for n = 0; n < 100 && a == 0; n++ {
		for v = 0; v < 100; v++ {
			m := convertToMap(strArr)
			ans = start(m, n, v)

			if ans == 19690720 {
				fmt.Println(ans, n, v)
				fmt.Println(100*n + v)

				a = 99
				break
			}
		}
	}
}

func start(codes map[int]int, noun int, verb int) int {
	codes[1] = noun
	codes[2] = verb

	doCalc(codes)

	return codes[0]
}

func doCalc(m map[int]int) {
	k := 0

	for k != 99 {
		switch m[k] {
		case 1:
			a1 := m[k+1]
			a2 := m[k+2]
			dest := m[k+3]

			m[dest] = m[a1] + m[a2]

			k += 4
		case 2:
			a1 := m[k+1]
			a2 := m[k+2]
			dest := m[k+3]

			m[dest] = m[a1] * m[a2]

			k += 4
		case 99:
			k = 99
		}
	}
}

func convertToMap(str []string) map[int]int {
	m := make(map[int]int)
	for i, v := range str {
		num, _ := strconv.Atoi(v)
		m[i] = num
	}

	return m
}
