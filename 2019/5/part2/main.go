package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	input, _ := ioutil.ReadFile("../input.txt")
	txt := strings.TrimSpace(string(input))

	strArr := strings.Split(txt, ",")
	m := convertToMap(strArr)

	doCalc(m)
}

func doCalc(m map[int]int) {
	k := 0

	for k != 99 {
		n := m[k]
		oc := n % 100
		n /= 100
		p1m := n % 10
		n /= 10
		p2m := n % 10

		switch oc {
		case 1:
			p1 := m[k+1]
			if p1m == 0 {
				p1 = m[p1]
			}

			p2 := m[k+2]
			if p2m == 0 {
				p2 = m[p2]
			}

			p3 := m[k+3]
			m[p3] = p1 + p2

			k += 4
		case 2:
			p1 := m[k+1]
			if p1m == 0 {
				p1 = m[p1]
			}

			p2 := m[k+2]
			if p2m == 0 {
				p2 = m[p2]
			}

			p3 := m[k+3]
			m[p3] = p1 * p2

			k += 4
		case 3:
			fmt.Print("Enter number: ")
			var in int
			fmt.Scanf("%d", &in)

			m[m[k+1]] = in

			k += 2
		case 4:
			p1 := m[k+1]
			if p1m == 0 {
				p1 = m[p1]
			}

			fmt.Println(p1)

			k += 2
		case 5:
			p1 := m[k+1]
			if p1m == 0 {
				p1 = m[p1]
			}

			p2 := m[k+2]
			if p2m == 0 {
				p2 = m[p2]
			}

			if p1 != 0 {
				k = p2
			} else {
				k += 3
			}
		case 6:
			p1 := m[k+1]
			if p1m == 0 {
				p1 = m[p1]
			}

			p2 := m[k+2]
			if p2m == 0 {
				p2 = m[p2]
			}

			if p1 == 0 {
				k = p2
			} else {
				k += 3
			}
		case 7:
			p1 := m[k+1]
			if p1m == 0 {
				p1 = m[p1]
			}

			p2 := m[k+2]
			if p2m == 0 {
				p2 = m[p2]
			}

			p3 := m[k+3]

			if p1 < p2 {
				m[p3] = 1
			} else {
				m[p3] = 0
			}

			k += 4
		case 8:
			p1 := m[k+1]
			if p1m == 0 {
				p1 = m[p1]
			}

			p2 := m[k+2]
			if p2m == 0 {
				p2 = m[p2]
			}

			p3 := m[k+3]

			if p1 == p2 {
				m[p3] = 1
			} else {
				m[p3] = 0
			}

			k += 4
		case 99:
			k = 99
		default:
			fmt.Println(k, m[k])
			panic(k)
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
